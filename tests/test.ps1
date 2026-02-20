<#
.SYNOPSIS
    HL7 MLLP Test Suite (Functional & Load Testing)
#>

$Server = "127.0.0.1"
$Port = 2575

# MLLP control characters
$SB = [char]0x0B # Start Block <VT>
$EB = [char]0x1C # End Block <FS>
$CR = [char]0x0D # Carriage Return <CR>

# ==========================================
# Functions
# ==========================================

function Send-Hl7Message {
    param (
        [string]$TestName,
        [string]$Payload
    )

    Write-Host "=> Sending $TestName to $Server:$Port" -ForegroundColor Cyan

    # HL7 segments must be separated by \r (Carriage Return), not \n.
    $formattedPayload = $Payload.Trim() -replace "`r?`n", $CR
    $mllpMessage = "$SB$formattedPayload$EB$CR"
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($mllpMessage)

    try {
        $tcpClient = New-Object System.Net.Sockets.TcpClient($Server, $Port)
        $stream = $tcpClient.GetStream()

        # Send
        $stream.Write($bytes, 0, $bytes.Length)
        Write-Host "   [OK] Message successfully sent to socket." -ForegroundColor Green

        # Receive ACK
        $buffer = New-Object byte[] 4096
        # Note: We wait briefly to ensure the server responds. 
        # In a robust production client, you'd loop until $EB is received.
        Start-Sleep -Milliseconds 200 
        
        if ($stream.DataAvailable) {
            $bytesRead = $stream.Read($buffer, 0, $buffer.Length)
            $response = [System.Text.Encoding]::UTF8.GetString($buffer, 0, $bytesRead)
            
            # Strip MLLP wrappers and make readable
            $cleanResponse = $response.Replace($SB, '').Replace($EB, '').Replace($CR, "`n")
            Write-Host "   [SERVER-ACK]:`n$cleanResponse"
        } else {
            Write-Host "   [SERVER-ACK]: (No response received within timeout)" -ForegroundColor Yellow
        }

        $tcpClient.Close()
    }
    catch {
        Write-Host "   [ERROR] Connection failed: $($_.Exception.Message)" -ForegroundColor Red
    }
    Write-Host "--------------------------------------------------"
    Start-Sleep -Seconds 1
}

function Invoke-Hl7LoadTest {
    param (
        [string]$Payload,
        [int]$Iterations
    )

    Write-Host "=================================================="
    Write-Host "Starting Load Test: Sending $Iterations messages..." -ForegroundColor Cyan
    Write-Host "Using persistent TCP connection for accurate throughput." -ForegroundColor DarkCyan
    Write-Host "=================================================="

    try {
        $tcpClient = New-Object System.Net.Sockets.TcpClient($Server, $Port)
        $stream = $tcpClient.GetStream()
        
        $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()

        for ($i = 1; $i -le $Iterations; $i++) {
            # Modify Control ID to prevent server-side duplicate dropping
            $currentPayload = $Payload.Trim() -replace "MSG0001", "LOAD$($i.ToString('D4'))"
            $formattedPayload = $currentPayload -replace "`r?`n", $CR
            
            $mllpMessage = "$SB$formattedPayload$EB$CR"
            $bytes = [System.Text.Encoding]::UTF8.GetBytes($mllpMessage)

            $stream.Write($bytes, 0, $bytes.Length)

            # Minimal read to clear the buffer so the server doesn't block (ignoring content for speed)
            if ($stream.DataAvailable) {
                $nullBuffer = New-Object byte[] 2048
                $null = $stream.Read($nullBuffer, 0, $nullBuffer.Length)
            }

            if ($i % 25 -eq 0) {
                Write-Host "  ...sent $i messages"
            }
        }

        $stopwatch.Stop()
        $tcpClient.Close()

        $duration = $stopwatch.Elapsed.TotalSeconds
        $throughput = $Iterations / $duration

        Write-Host "--------------------------------------------------"
        Write-Host "Load Test completed." -ForegroundColor Green
        Write-Host "Sent $Iterations messages in $([math]::Round($duration, 2)) seconds."
        Write-Host "Approximate Throughput: $([math]::Round($throughput, 2)) messages/second." -ForegroundColor Cyan
        Write-Host "=================================================="

    }
    catch {
        Write-Host "[ERROR] Load test failed: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# ==========================================
# HL7 Dummy Messages (Valid)
# ==========================================

$MSG_ADT = @"
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220131200||ADT^A01|MSG0001|P|2.3
EVN|A01|20260220131200
PID|1||1001^^^HOSP^MR||Muster^Max^M||19800101|M|||Teststraße 1^^Konstanz^BW^78462^DE||0151-12345678|||M
PV1|1|I|INTENSIV^Bett 1^Zimmer 3||||1234^Arzt^Andreas
"@

$MSG_ORU = @"
MSH|^~\&|LAB_APP|LAB_FAC|RECV_APP|RECV_FAC|20260220131500||ORU^R01|MSG0002|P|2.3
PID|1||1002^^^HOSP^MR||Doe^Jane||19920515|F
OBR|1||54321^LAB|WBC^White Blood Count|||20260220130000
OBX|1|NM|WBC^White Blood Count||7.5|10*3/uL|4.0-10.0|N|||F
"@

$MSG_SIU = @"
MSH|^~\&|SCHED_APP|CLINIC|RECV_APP|RECV_FAC|20260220132000||SIU^S12|MSG0003|P|2.3
SCH|112233|223344||||ROUTINE|Kontrolle|MINS|30|m||||20260301100000
PID|1||1003^^^HOSP^MR||Schmidt^Anna||19751120|F
"@

# ==========================================
# HL7 Dummy Messages (Faulty)
# ==========================================

$MSG_ERR_NO_MSH = @"
PID|1||1004^^^HOSP^MR||Fehler^OhneMSH||19900101|M
PV1|1|O|||||9999^Unbekannt
"@

$MSG_ERR_INV_TYPE = @"
MSH|^~\&|SEND_APP|SEND_FAC|RECV_APP|RECV_FAC|20260220132500||ZZZ^Z99|MSG0004|P|2.3
PID|1||1005^^^HOSP^MR||Test^Unbekannt||19850101|M
"@

$MSG_ERR_BAD_MSH = @"
MSH|^~\&|||||20260220133000||ADT^A04||P|2.3
PID|1||1006^^^HOSP^MR||Test^Kaputt||19700101|F
"@

# ==========================================
# Execution
# ==========================================

Write-Host "Starting HL7 MLLP functional tests against $Server:$Port..."
Write-Host "=================================================="

Send-Hl7Message "ADT^A01 (Valid Patient admission)" $MSG_ADT
Send-Hl7Message "ORU^R01 (Valid Lab result)" $MSG_ORU
Send-Hl7Message "SIU^S12 (Valid Appointment booking)" $MSG_SIU

Write-Host "Running ERROR Handling Tests..." -ForegroundColor Yellow
Write-Host "--------------------------------------------------"
Send-Hl7Message "ERROR: Missing MSH Segment" $MSG_ERR_NO_MSH
Send-Hl7Message "ERROR: Unknown Message Type (ZZZ^Z99)" $MSG_ERR_INV_TYPE
Send-Hl7Message "ERROR: Missing Mandatory Fields in MSH" $MSG_ERR_BAD_MSH

Write-Host "Functional test run completed.`n"

# Run Load Test (1000 iterations for a better test sample since TCP is persistent)
Invoke-Hl7LoadTest -Payload $MSG_ADT -Iterations 1000