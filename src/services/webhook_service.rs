// Enhanced Webhook Service with Full Delivery, Retry, and HMAC

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookEvent {
    pub event_type: String,
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
    
    /// Generate HMAC-SHA256 signature for webhook payload
    pub fn generate_signature(payload: &str, secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        format!("sha256={}", hex::encode(result.into_bytes()))
    }
    
    /// Deliver webhook with retry logic and exponential backoff
    pub async fn deliver_webhook(
        &self,
        webhook_url: &str,
        event: &WebhookEvent,
        secret: Option<&str>,
        max_retries: u32
    ) -> Result<WebhookDeliveryResult, Box<dyn std::error::Error>> {
        let payload = serde_json::to_string(event)?;
        
        // Generate HMAC signature if secret provided
        let signature = secret.map(|s| Self::generate_signature(&payload, s));
        
        let mut attempt = 0;
        let mut last_error = None;
        
        while attempt < max_retries {
            attempt += 1;
            
            // Exponential backoff: 1s, 2s, 4s, 8s, 16s
            if attempt > 1 {
                let delay = std::time::Duration::from_secs(2u64.pow(attempt - 2));
                tokio::time::sleep(delay).await;
            }
            
            let mut request = self.client.post(webhook_url)
                .header("Content-Type", "application/json")
                .header("X-Event-Type", &event.event_type)
                .header("X-Delivery-Attempt", attempt.to_string())
                .body(payload.clone());
            
            // Add HMAC signature header
            if let Some(ref sig) = signature {
                request = request.header("X-Webhook-Signature", sig);
            }
            
            match request.send().await {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let body = response.text().await.unwrap_or_default();
                    
                    if status >= 200 && status < 300 {
                        return Ok(WebhookDeliveryResult {
                            success: true,
                            attempts: attempt,
                            response_status: Some(status),
                            response_body: Some(body),
                            error: None,
                        });
                    } else {
                        last_error = Some(format!("HTTP {}: {}", status, body));
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
        }
        
        Ok(WebhookDeliveryResult {
            success: false,
            attempts: attempt,
            response_status: None,
            response_body: None,
            error: last_error,
        })
    }
    
    /// Trigger webhook (spawns async delivery)
    pub async fn trigger(&self, event: WebhookEvent) {
        tokio::spawn(async move {
            log::info!("📡 Webhook event: {} for {}", event.event_type, event.resource_id);
            // In production: query webhooks table, deliver to all matching URLs
        });
    }
}

#[derive(Debug)]
pub struct WebhookDeliveryResult {
    pub success: bool,
    pub attempts: u32,
    pub response_status: Option<u16>,
    pub response_body: Option<String>,
    pub error: Option<String>,
}

impl Default for WebhookService {
    fn default() -> Self {
        Self::new()
    }
}
