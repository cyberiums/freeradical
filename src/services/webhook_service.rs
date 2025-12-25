// Webhook Service - Event system with HTTP delivery

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookEvent {
    pub event_type: String,  // "page.created", "page.updated", etc.
    pub resource_type: String,
    pub resource_id: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct WebhookService {
    client: reqwest::Client,
}

impl WebhookService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }
    
    /// Trigger webhooks for an event (async background task)
    pub async fn trigger(&self, event: WebhookEvent) {
        // In production, this should be queued to a background worker
        // For now, we'll fire-and-forget
        
        tokio::spawn(async move {
            log::info!("📡 Webhook event: {} for {}", event.event_type, event.resource_id);
            
            // In a full implementation:
            // 1. Query webhooks table for matching events
            // 2. Send HTTP POST to each webhook URL
            // 3. Log results to webhook_logs
            // 4. Retry on failure with exponential backoff
            // 5. Disable webhook after 10 consecutive failures
        });
    }
}

impl Default for WebhookService {
    fn default() -> Self {
        Self::new()
    }
}
