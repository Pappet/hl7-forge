use crate::hl7::types::{Hl7Message, Hl7MessageSummary};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::info;

// Size-based eviction (MAX_STORE_BYTES) is the primary safeguard for large messages
// (e.g. MDM with Base64). Count limit is a secondary backstop.
const DEFAULT_CAPACITY: usize = 10_000;
const MAX_STORE_BYTES: usize = 512 * 1024 * 1024; // 512 MB
const BROADCAST_CAPACITY: usize = 4096;

#[derive(Clone)]
pub enum StoreEvent {
    NewMessage(Box<Hl7MessageSummary>),
    Cleared,
}

/// Thread-safe in-memory message store with broadcast notifications
#[derive(Clone)]
pub struct MessageStore {
    inner: Arc<RwLock<StoreInner>>,
    tx: broadcast::Sender<StoreEvent>,
}

struct StoreInner {
    messages: VecDeque<Hl7Message>,
    capacity: usize,
    current_bytes: usize,
}

impl MessageStore {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            inner: Arc::new(RwLock::new(StoreInner {
                messages: VecDeque::with_capacity(1024),
                capacity: DEFAULT_CAPACITY,
                current_bytes: 0,
            })),
            tx,
        }
    }

    /// Insert a message and broadcast summary to all WebSocket subscribers
    pub async fn insert(&self, msg: Hl7Message) {
        let summary = Hl7MessageSummary::from(&msg);

        let mut inner = self.inner.write().await;

        // Evict oldest 10% when either size or count limit is breached
        if inner.current_bytes >= MAX_STORE_BYTES || inner.messages.len() >= inner.capacity {
            let drain_count = inner.messages.len() / 10;
            let freed_bytes: usize = inner
                .messages
                .iter()
                .take(drain_count)
                .map(|m| m.raw.len())
                .sum();
            inner.messages.drain(..drain_count);
            inner.current_bytes = inner.current_bytes.saturating_sub(freed_bytes);
            info!(
                "Evicted {} messages from store ({} MB freed, store now {} messages / {} MB)",
                drain_count,
                freed_bytes / 1024 / 1024,
                inner.messages.len(),
                inner.current_bytes / 1024 / 1024,
            );
        }

        inner.current_bytes += msg.raw.len();
        inner.messages.push_back(msg);
        let count = inner.messages.len();
        drop(inner);

        // Broadcast to WebSocket subscribers (ignore if no receivers)
        let _ = self.tx.send(StoreEvent::NewMessage(Box::new(summary)));

        if count % 1000 == 0 {
            info!("Store now holds {} messages", count);
        }
    }

    /// Get a broadcast receiver for real-time updates
    pub fn subscribe(&self) -> broadcast::Receiver<StoreEvent> {
        self.tx.subscribe()
    }

    /// Get all message summaries (lightweight)
    pub async fn list_summaries(&self, offset: usize, limit: usize) -> Vec<Hl7MessageSummary> {
        let inner = self.inner.read().await;
        inner
            .messages
            .iter()
            .rev() // newest first
            .skip(offset)
            .take(limit)
            .map(Hl7MessageSummary::from)
            .collect()
    }

    /// Get a full message by ID
    pub async fn get_by_id(&self, id: &str) -> Option<Hl7Message> {
        let inner = self.inner.read().await;
        inner.messages.iter().find(|m| m.id == id).cloned()
    }

    /// Search messages by filter text (matches message type, patient name, facility, etc.)
    pub async fn search(&self, query: &str, limit: usize) -> Vec<Hl7MessageSummary> {
        let query_lower = query.to_lowercase();
        let inner = self.inner.read().await;
        inner
            .messages
            .iter()
            .rev()
            .filter(|m| {
                m.message_type.to_lowercase().contains(&query_lower)
                    || m.sending_facility.to_lowercase().contains(&query_lower)
                    || m.patient_name
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                    || m.patient_id
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                    || m.message_control_id.to_lowercase().contains(&query_lower)
                    || m.source_addr.contains(&query_lower)
            })
            .take(limit)
            .map(Hl7MessageSummary::from)
            .collect()
    }

    /// Total message count
    pub async fn count(&self) -> usize {
        self.inner.read().await.messages.len()
    }

    /// Clear all messages
    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.messages.clear();
        inner.current_bytes = 0;
        info!("Message store cleared");
        let _ = self.tx.send(StoreEvent::Cleared);
    }
}
