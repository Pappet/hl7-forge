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

# ==========================================
# HL7 Dummy Messages (fake patients)
# ==========================================

# 1. ADT^A01 (Patient admission)
read -r -d '' MSG_ADT << EOM
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220131200||ADT^A01|MSG0001|P|2.3
EVN|A01|20260220131200
PID|1||1001^^^HOSP^MR||Muster^Max^M||19800101|M|||Teststraße 1^^Konstanz^BW^78462^DE||0151-12345678|||M
PV1|1|I|INTENSIV^Bett 1^Zimmer 3||||1234^Arzt^Andreas
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
SCH|112233|223344||||ROUTINE|Kontrolle|MINS|30|m||||20260301100000
PID|1||1003^^^HOSP^MR||Schmidt^Anna||19751120|F
EOM

# ==========================================
# Run test suite
# ==========================================

echo "Starting HL7 MLLP test run against $HOST:$PORT..."
echo "--------------------------------------------------"

send_mllp "ADT^A01 (Patient admission)" "$MSG_ADT"
send_mllp "ORU^R01 (Lab result)" "$MSG_ORU"
send_mllp "SIU^S12 (Appointment booking)" "$MSG_SIU"

echo "Test run completed."
