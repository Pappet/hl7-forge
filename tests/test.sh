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

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MSG_DIR="$SCRIPT_DIR/messages"

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
# Run Functional Tests
# ==========================================

echo "Starting HL7 MLLP functional tests against $HOST:$PORT..."
echo "=================================================="

for msg_file in "$MSG_DIR/valid"/*.hl7; do
    [ -f "$msg_file" ] || continue
    label=$(basename "$msg_file" .hl7 | tr '_' ' ')
    send_mllp "$label" "$(cat "$msg_file")"
done

echo "Running ERROR Handling Tests..."
echo "--------------------------------------------------"

for msg_file in "$MSG_DIR/errors"/*.hl7; do
    [ -f "$msg_file" ] || continue
    label=$(basename "$msg_file" .hl7 | tr '_' ' ')
    send_mllp "ERROR: $label" "$(cat "$msg_file")"
done

echo "Functional test run completed."
echo ""

# ==========================================
# Run Load Test
# ==========================================

LOAD_ITERATIONS=100
LOAD_MSG=$(cat "$MSG_DIR/valid/adt_a01.hl7")

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
    CURRENT_MSG=$(echo "$LOAD_MSG" | sed "s/MSG0001/LOAD$(printf "%04d" $i)/")
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
