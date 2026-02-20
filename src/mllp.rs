use crate::hl7::parser::{build_ack, parse_message};
use crate::store::MessageStore;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::watch;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

/// MLLP framing constants
const MLLP_START: u8 = 0x0B; // Vertical Tab (VT)
const MLLP_END_1: u8 = 0x1C; // File Separator (FS)
const MLLP_END_2: u8 = 0x0D; // Carriage Return (CR)

const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10 MB hard limit per connection buffer
const READ_TIMEOUT: Duration = Duration::from_secs(60);
const WRITE_TIMEOUT: Duration = Duration::from_secs(30);

/// Stats for the MLLP server
#[derive(Clone)]
pub struct MllpStats {
    pub received: Arc<AtomicU64>,
    pub parsed_ok: Arc<AtomicU64>,
    pub parse_errors: Arc<AtomicU64>,
    pub active_connections: Arc<AtomicU64>,
}

impl MllpStats {
    pub fn new() -> Self {
        Self {
            received: Arc::new(AtomicU64::new(0)),
            parsed_ok: Arc::new(AtomicU64::new(0)),
            parse_errors: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
        }
    }
}

/// Start the MLLP TCP server
pub async fn start_mllp_server(
    bind_addr: &str,
    store: MessageStore,
    stats: MllpStats,
    mut shutdown: watch::Receiver<bool>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    info!("MLLP server listening on {}", bind_addr);

    loop {
        tokio::select! {
            biased;
            _ = shutdown.changed() => {
                info!("MLLP server shutting down gracefully");
                break;
            }
            result = listener.accept() => {
                let (socket, peer_addr) = result?;
                let store = store.clone();
                let stats = stats.clone();
                let peer = peer_addr.to_string();

                stats.active_connections.fetch_add(1, Ordering::Relaxed);
                info!("New MLLP connection from {}", peer);

                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket, &peer, &store, &stats).await {
                        warn!("Connection error from {}: {}", peer, e);
                    }
                    stats.active_connections.fetch_sub(1, Ordering::Relaxed);
                    info!("MLLP connection closed: {}", peer);
                });
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    peer: &str,
    store: &MessageStore,
    stats: &MllpStats,
) -> anyhow::Result<()> {
    let mut buf = vec![0u8; 64 * 1024]; // 64 KB read buffer
    let mut accumulated = Vec::with_capacity(8 * 1024);

    loop {
        let n = match timeout(READ_TIMEOUT, socket.read(&mut buf)).await {
            Ok(Ok(0)) | Err(_) => {
                // Connection closed or read timeout
                if accumulated.is_empty() {
                    break;
                }
                warn!("Read timeout or connection closed from {}", peer);
                break;
            }
            Ok(Ok(n)) => n,
            Ok(Err(e)) => return Err(e.into()),
        };

        accumulated.extend_from_slice(&buf[..n]);

        if accumulated.len() > MAX_MESSAGE_SIZE {
            warn!(
                "Buffer exceeded {} MB from {}, closing connection",
                MAX_MESSAGE_SIZE / 1024 / 1024,
                peer
            );
            return Ok(());
        }

        // Process all complete MLLP frames in the buffer
        while let Some((message, consumed)) = extract_mllp_frame(&accumulated) {
            stats.received.fetch_add(1, Ordering::Relaxed);

            match parse_message(&message, peer) {
                Ok(msg) => {
                    stats.parsed_ok.fetch_add(1, Ordering::Relaxed);

                    // Build and send ACK
                    let ack = build_ack(&msg, "AA");
                    let ack_frame = wrap_mllp(&ack);
                    match timeout(WRITE_TIMEOUT, socket.write_all(&ack_frame)).await {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => warn!("Failed to send ACK to {}: {}", peer, e),
                        Err(_) => warn!("Write timeout sending ACK to {}", peer),
                    }

                    // Store the message (async, non-blocking for the connection)
                    store.insert(msg).await;
                }
                Err(e) => {
                    stats.parse_errors.fetch_add(1, Ordering::Relaxed);
                    warn!("Parse error from {}: {}", peer, e);

                    // Send NACK (AE = Application Error)
                    let nack =
                        "MSH|^~\\&|HL7Forge|HL7Forge|||||ACK||P|2.5\rMSA|AE|UNKNOWN|Message parse error"
                            .to_string();
                    let nack_frame = wrap_mllp(&nack);
                    let _ = timeout(WRITE_TIMEOUT, socket.write_all(&nack_frame)).await;
                }
            }

            // Remove processed bytes
            accumulated.drain(..consumed);
        }
    }

    Ok(())
}

/// Extract one complete MLLP frame from the buffer.
/// Returns (message_content, bytes_consumed) or None if incomplete.
fn extract_mllp_frame(buf: &[u8]) -> Option<(String, usize)> {
    // Find start byte
    let start_pos = buf.iter().position(|&b| b == MLLP_START)?;

    // Find end sequence (FS + CR)
    for i in (start_pos + 1)..buf.len().saturating_sub(1) {
        if buf[i] == MLLP_END_1 && buf[i + 1] == MLLP_END_2 {
            let message_bytes = &buf[start_pos + 1..i];
            let message = String::from_utf8_lossy(message_bytes).to_string();
            return Some((message, i + 2));
        }
    }

    None // Incomplete frame
}

/// Wrap a message in MLLP framing
fn wrap_mllp(message: &str) -> Vec<u8> {
    let mut frame = Vec::with_capacity(message.len() + 3);
    frame.push(MLLP_START);
    frame.extend_from_slice(message.as_bytes());
    frame.push(MLLP_END_1);
    frame.push(MLLP_END_2);
    frame
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mllp_frame() {
        let msg = "MSH|^~\\&|TEST\rPID|||123";
        let mut frame = vec![MLLP_START];
        frame.extend_from_slice(msg.as_bytes());
        frame.push(MLLP_END_1);
        frame.push(MLLP_END_2);

        let (extracted, consumed) = extract_mllp_frame(&frame).unwrap();
        assert_eq!(extracted, msg);
        assert_eq!(consumed, frame.len());
    }

    #[test]
    fn test_incomplete_frame() {
        let frame = vec![MLLP_START, b'M', b'S', b'H'];
        assert!(extract_mllp_frame(&frame).is_none());
    }

    #[test]
    fn test_wrap_mllp() {
        let wrapped = wrap_mllp("TEST");
        assert_eq!(wrapped[0], MLLP_START);
        assert_eq!(wrapped[wrapped.len() - 2], MLLP_END_1);
        assert_eq!(wrapped[wrapped.len() - 1], MLLP_END_2);
    }
}
