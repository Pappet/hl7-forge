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

$MsgDir = Join-Path $PSScriptRoot "messages"

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
# Run Functional Tests
# ==========================================

Write-Host "Starting HL7 MLLP functional tests against $Server:$Port..."
Write-Host "=================================================="

Get-ChildItem "$MsgDir\valid\*.hl7" | Sort-Object Name | ForEach-Object {
    $label = $_.BaseName -replace '_', ' '
    Send-Hl7Message $label (Get-Content $_.FullName -Raw)
}

Write-Host "Running ERROR Handling Tests..." -ForegroundColor Yellow
Write-Host "--------------------------------------------------"

Get-ChildItem "$MsgDir\errors\*.hl7" | Sort-Object Name | ForEach-Object {
    $label = "ERROR: $($_.BaseName -replace '_', ' ')"
    Send-Hl7Message $label (Get-Content $_.FullName -Raw)
}

Write-Host "Functional test run completed.`n"

# Run Load Test (1000 iterations for a better test sample since TCP is persistent)
$loadPayload = Get-Content "$MsgDir\valid\adt_a01.hl7" -Raw
Invoke-Hl7LoadTest -Payload $loadPayload -Iterations 1000
