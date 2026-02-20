use crate::hl7::types::{Hl7Message, Hl7MessageSummary};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::info;

const DEFAULT_CAPACITY: usize = 100_000;
const BROADCAST_CAPACITY: usize = 4096;

/// Thread-safe in-memory message store with broadcast notifications
#[derive(Clone)]
pub struct MessageStore {
    inner: Arc<RwLock<StoreInner>>,
    tx: broadcast::Sender<Hl7MessageSummary>,
}

struct StoreInner {
    messages: Vec<Hl7Message>,
    capacity: usize,
}

impl MessageStore {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            inner: Arc::new(RwLock::new(StoreInner {
                messages: Vec::with_capacity(1024),
                capacity: DEFAULT_CAPACITY,
            })),
            tx,
        }
    }

    /// Insert a message and broadcast summary to all WebSocket subscribers
    pub async fn insert(&self, msg: Hl7Message) {
        let summary = Hl7MessageSummary::from(&msg);

        let mut inner = self.inner.write().await;

        // Evict oldest messages if at capacity
        if inner.messages.len() >= inner.capacity {
            let drain_count = inner.capacity / 10; // remove 10%
            inner.messages.drain(..drain_count);
            info!("Evicted {} old messages from store", drain_count);
        }

        inner.messages.push(msg);
        let count = inner.messages.len();
        drop(inner);

        // Broadcast to WebSocket subscribers (ignore if no receivers)
        let _ = self.tx.send(summary);

        if count % 1000 == 0 {
            info!("Store now holds {} messages", count);
        }
    }

    /// Get a broadcast receiver for real-time updates
    pub fn subscribe(&self) -> broadcast::Receiver<Hl7MessageSummary> {
        self.tx.subscribe()
    }

    /// Get all message summaries (lightweight)
    pub async fn list_summaries(&self, offset: usize, limit: usize) -> Vec<Hl7MessageSummary> {
        let inner = self.inner.read().await;
        inner.messages
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
        inner.messages
            .iter()
            .rev()
            .filter(|m| {
                m.message_type.to_lowercase().contains(&query_lower)
                    || m.sending_facility.to_lowercase().contains(&query_lower)
                    || m.patient_name.as_deref().unwrap_or("").to_lowercase().contains(&query_lower)
                    || m.patient_id.as_deref().unwrap_or("").to_lowercase().contains(&query_lower)
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
        self.inner.write().await.messages.clear();
        info!("Message store cleared");
    }
}
