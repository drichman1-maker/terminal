use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;

/// IPC Bus for streaming Cap'n Proto data to frontend
pub struct IPCBus {
    subscribers: Arc<DashMap<String, Vec<String>>>, // topic -> window labels
}

impl IPCBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(DashMap::new()),
        }
    }

    /// Subscribe a window to a topic
    pub fn subscribe(&self, topic: String, window_label: String) {
        self.subscribers
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(window_label);
    }

    /// Unsubscribe a window from a topic
    pub fn unsubscribe(&self, topic: &str, window_label: &str) {
        if let Some(mut subscribers) = self.subscribers.get_mut(topic) {
            subscribers.retain(|label| label != window_label);
        }
    }

    /// Broadcast Cap'n Proto binary data to all subscribers of a topic
    pub async fn broadcast(&self, topic: &str, data: Vec<u8>) {
        if let Some(subscribers) = self.subscribers.get(topic) {
            for window_label in subscribers.iter() {
                tracing::debug!("Broadcasting to window: {}", window_label);
                // TODO: Emit Tauri event with binary payload
            }
        }
    }
}
