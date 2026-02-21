#!/bin/bash

HOST="localhost"
PORT=2575

# Check if nc (netcat) is available
if ! command -v nc &> /dev/null; then
    echo "Error: 'nc' (netcat) is not installed."
    exit 1
fi

# MLLP control characters (hex values)
SB=$(printf '\x0b') # Start Block <VT>
EB=$(printf '\x1c') # End Block <FS>
CR=$(printf '\x0d') # Carriage Return <CR>

send_mllp() {
    local msg_type="$1"
    local payload="$2"

    echo "=> Sending $msg_type to $HOST:$PORT"

    # HL7 segments must be separated by \r (Carriage Return), not \n.
    # sed removes empty lines, tr converts \n to \r.
    formatted_payload=$(echo "$payload" | sed '/^$/d' | tr '\n' '\r')

    # Send payload.
    # %b interprets binary strings (SB/EB/CR), %s passes the HL7 text raw
    # to avoid conflicts with HL7 delimiters (like \) during printf evaluation.
    # nc -w 2 times out after 2 seconds without traffic.
    response=$(printf "%b%s%b%b" "$SB" "$formatted_payload" "$EB" "$CR" | nc -w 2 "$HOST" "$PORT")

    if [ $? -eq 0 ]; then
        echo "   [OK] Message successfully sent to socket."
        # Strip MLLP wrapper from server response and make \r readable for terminal
        clean_response=$(echo "$response" | tr -d '\013\034' | tr '\r' '\n')
        if [ -n "$clean_response" ]; then
            echo -e "   [SERVER-ACK]:\n$clean_response"
        else
            echo "   [SERVER-ACK]: (No response received)"
        fi
    else
        echo "   [ERROR] Connection failed. Is the server running at $HOST:$PORT?"
    fi
    echo "--------------------------------------------------"
    sleep 1
}

send_mllp_fast() {
    # Faster version without echoes and sleeps for load testing.
    # Still incurs process-spawning overhead from 'nc'.
    local payload="$1"
    formatted_payload=$(echo "$payload" | sed '/^$/d' | tr '\n' '\r')
    printf "%b%s%b%b" "$SB" "$formatted_payload" "$EB" "$CR" | nc -w 1 "$HOST" "$PORT" > /dev/null 2>&1
}

# ==========================================
# HL7 Dummy Messages (Valid)
# ==========================================

# 1. ADT^A01 (Patient admission)
read -r -d '' MSG_ADT << EOM
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220131200||ADT^A01|MSG0001|P|2.3
EVN|A01|20260220131200
PID|1||1001^^^HOSP^MR||Sample^Max^M||19800101|M|||1 Test Street^^Springfield^IL^62701^US||555-12345678|||M
PV1|1|I|ICU^Bed 1^Room 3||||1234^Doctor^Andrew
EOM

# 2. ORU^R01 (Lab result)
read -r -d '' MSG_ORU << EOM
MSH|^~\&|LAB_APP|LAB_FAC|RECV_APP|RECV_FAC|20260220131500||ORU^R01|MSG0002|P|2.3
PID|1||1002^^^HOSP^MR||Doe^Jane||19920515|F
OBR|1||54321^LAB|WBC^White Blood Count|||20260220130000
OBX|1|NM|WBC^White Blood Count||7.5|10*3/uL|4.0-10.0|N|||F
EOM

# 3. SIU^S12 (Appointment booking)
read -r -d '' MSG_SIU << EOM
MSH|^~\&|SCHED_APP|CLINIC|RECV_APP|RECV_FAC|20260220132000||SIU^S12|MSG0003|P|2.3
SCH|112233|223344||||ROUTINE|Checkup|MINS|30|m||||20260301100000
PID|1||1003^^^HOSP^MR||Smith^Anna||19751120|F
EOM

# ==========================================
# HL7 Dummy Messages (Faulty)
# ==========================================

# 4. Missing MSH Segment (Critical structural error)
read -r -d '' MSG_ERR_NO_MSH << EOM
PID|1||1004^^^HOSP^MR||Error^NoMSH||19900101|M
PV1|1|O|||||9999^Unknown
EOM

# 5. Invalid/Unknown Message Type (ZZZ^Z99)
read -r -d '' MSG_ERR_INV_TYPE << EOM
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220132500||ZZZ^Z99|MSG0004|P|2.3
PID|1||1005^^^HOSP^MR||Test^Unknown||19850101|M
EOM

# 6. Missing Mandatory Fields in MSH (Missing Message Control ID)
read -r -d '' MSG_ERR_BAD_MSH << EOM
MSH|^~\&|||||20260220133000||ADT^A04||P|2.3
PID|1||1006^^^HOSP^MR||Test^Broken||19700101|F
EOM


# ==========================================
# Run Functional Tests
# ==========================================

echo "Starting HL7 MLLP functional tests against $HOST:$PORT..."
echo "=================================================="

send_mllp "ADT^A01 (Valid Patient admission)" "$MSG_ADT"
send_mllp "ORU^R01 (Valid Lab result)" "$MSG_ORU"
send_mllp "SIU^S12 (Valid Appointment booking)" "$MSG_SIU"

echo "Running ERROR Handling Tests..."
echo "--------------------------------------------------"
send_mllp "ERROR: Missing MSH Segment" "$MSG_ERR_NO_MSH"
send_mllp "ERROR: Unknown Message Type (ZZZ^Z99)" "$MSG_ERR_INV_TYPE"
send_mllp "ERROR: Missing Mandatory Fields in MSH" "$MSG_ERR_BAD_MSH"

echo "Functional test run completed."
echo ""

# ==========================================
# Run Load Test
# ==========================================

LOAD_ITERATIONS=100

echo "=================================================="
echo "Starting Load Test: Sending $LOAD_ITERATIONS ADT^A01 messages..."
echo "=================================================="

# Check if 'bc' is installed for floating point math
if ! command -v bc &> /dev/null; then
    echo "Warning: 'bc' is not installed. Throughput calculation will be skipped."
    HAS_BC=0
else
    HAS_BC=1
fi

START_TIME=$(date +%s)

for ((i=1; i<=LOAD_ITERATIONS; i++)); do
    # Generate a unique Control ID for each message to avoid server-side caching/duplication drops
    CURRENT_MSG=$(echo "$MSG_ADT" | sed "s/MSG0001/LOAD$(printf "%04d" $i)/")
    send_mllp_fast "$CURRENT_MSG"
    
    # Progress indicator
    if (( i % 25 == 0 )); then
        echo "  ...sent $i messages"
    fi
done

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "--------------------------------------------------"
echo "Load Test completed."
echo "Sent $LOAD_ITERATIONS messages in $DURATION seconds."

if [ "$DURATION" -gt 0 ] && [ "$HAS_BC" -eq 1 ]; then
    MSG_PER_SEC=$(echo "scale=2; $LOAD_ITERATIONS / $DURATION" | bc)
    echo "Approximate Throughput: $MSG_PER_SEC messages/second."
elif [ "$DURATION" -eq 0 ]; then
    echo "Test completed in under 1 second."
fi
echo "=================================================="
