#!/bin/bash

HOST="localhost"
PORT=2575

# Prüfen, ob nc (netcat) verfügbar ist
if ! command -v nc &> /dev/null; then
    echo "Fehler: 'nc' (netcat) ist nicht installiert."
    exit 1
fi

# MLLP Steuerzeichen (Hex-Werte)
SB=$(printf '\x0b') # Start Block <VT>
EB=$(printf '\x1c') # End Block <FS>
CR=$(printf '\x0d') # Carriage Return <CR>

send_mllp() {
    local msg_type="$1"
    local payload="$2"

    echo "=> Sende $msg_type an $HOST:$PORT"

    # HL7 Segmente müssen mit \r (Carriage Return) getrennt werden, nicht mit \n.
    # sed löscht leere Zeilen, tr wandelt \n in \r um.
    formatted_payload=$(echo "$payload" | sed '/^$/d' | tr '\n' '\r')
    
    # Payload senden. 
    # %b interpretiert Binär-Strings (SB/EB/CR), %s übergibt den HL7-Text roh, 
    # um Konflikte mit HL7-Trennzeichen (wie \) bei der printf-Auswertung zu vermeiden.
    # nc -w 2 bricht nach 2 Sekunden ohne Traffic ab.
    response=$(printf "%b%s%b%b" "$SB" "$formatted_payload" "$EB" "$CR" | nc -w 2 "$HOST" "$PORT")
    
    if [ $? -eq 0 ]; then
        echo "   [OK] Nachricht erfolgreich an Socket übergeben."
        # MLLP-Wrapper aus der Server-Antwort entfernen und \r für das Terminal lesbar machen
        clean_response=$(echo "$response" | tr -d '\013\034' | tr '\r' '\n')
        if [ -n "$clean_response" ]; then
            echo -e "   [SERVER-ACK]:\n$clean_response"
        else
            echo "   [SERVER-ACK]: (Keine Antwort empfangen)"
        fi
    else
        echo "   [FEHLER] Verbindung fehlgeschlagen. Ist der Server unter $HOST:$PORT erreichbar?"
    fi
    echo "--------------------------------------------------"
    sleep 1
}

# ==========================================
# HL7 Dummy Nachrichten (Fake Patienten)
# ==========================================

# 1. ADT^A01 (Patientenaufnahme)
read -r -d '' MSG_ADT << EOM
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220131200||ADT^A01|MSG0001|P|2.3
EVN|A01|20260220131200
PID|1||1001^^^HOSP^MR||Muster^Max^M||19800101|M|||Teststraße 1^^Konstanz^BW^78462^DE||0151-12345678|||M
PV1|1|I|INTENSIV^Bett 1^Zimmer 3||||1234^Arzt^Andreas
EOM

# 2. ORU^R01 (Laborbefund)
read -r -d '' MSG_ORU << EOM
MSH|^~\&|LAB_APP|LAB_FAC|RECV_APP|RECV_FAC|20260220131500||ORU^R01|MSG0002|P|2.3
PID|1||1002^^^HOSP^MR||Doe^Jane||19920515|F
OBR|1||54321^LAB|WBC^White Blood Count|||20260220130000
OBX|1|NM|WBC^White Blood Count||7.5|10*3/uL|4.0-10.0|N|||F
EOM

# 3. SIU^S12 (Terminbuchung)
read -r -d '' MSG_SIU << EOM
MSH|^~\&|SCHED_APP|CLINIC|RECV_APP|RECV_FAC|20260220132000||SIU^S12|MSG0003|P|2.3
SCH|112233|223344||||ROUTINE|Kontrolle|MINS|30|m||||20260301100000
PID|1||1003^^^HOSP^MR||Schmidt^Anna||19751120|F
EOM

# ==========================================
# Ausführung des Testlaufs
# ==========================================

echo "Starte HL7 MLLP Testlauf gegen $HOST:$PORT..."
echo "--------------------------------------------------"

send_mllp "ADT^A01 (Patientenaufnahme)" "$MSG_ADT"
send_mllp "ORU^R01 (Laborbefund)" "$MSG_ORU"
send_mllp "SIU^S12 (Terminbuchung)" "$MSG_SIU"

echo "Testlauf abgeschlossen."
