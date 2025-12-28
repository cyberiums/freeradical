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
    pub async fn trigger(
        &self, 
        pool: &actix_web::web::Data<crate::models::DbPool>,
        tenant_id: i32,
        event_type: &str,
        resource_type: &str,
        resource_id: &str,
        data: serde_json::Value
    ) {
        use diesel::prelude::*;
        use crate::schema::tenant_webhooks;
        use crate::models::webhook_models::TenantWebhook;

        let pool = pool.clone();
        let event_type = event_type.to_string();
        let resource_type = resource_type.to_string();
        let resource_id = resource_id.to_string();
        // Clone data for the async block
        let data = data.clone();
        // We need a cloned client for the async task if we want to use self methods, 
        // but self is a reference. Easier to clone the client or Arc<Self>.
        // Since we are inside a method, let's just clone the client directly if possible or create a new one.
        // Actually, let's make this method require a lightweight cloneable service handle or just re-instantiate client.
        // For efficiency, we should fix the struct to be shareable, but for now, we'll just new() a client inside spawn if needed,
        // or better, Arc the service. 
        // SIMPLIFICATION: We will query first, then spawn tasks.
        
        let hooks_result = actix_web::web::block(move || {
            let mut conn = pool.get().expect("db conn");
            tenant_webhooks::table
                .filter(tenant_webhooks::tenant_id.eq(tenant_id))
                .filter(tenant_webhooks::is_active.eq(true))
                .load::<TenantWebhook>(&mut conn)
        }).await;

        if let Ok(Ok(hooks)) = hooks_result {
            for hook in hooks {
                // Check if hook subscribes to this event
                // events is JSONB array: ["order.created", ...]
                let subscribes = if let Some(arr) = hook.events.as_array() {
                    arr.iter().any(|v: &serde_json::Value| v.as_str() == Some(event_type.as_str()) || v.as_str() == Some("*"))
                } else {
                    false
                };

                if subscribes {
                    let evt = WebhookEvent {
                        event_type: event_type.clone(),
                        resource_type: resource_type.clone(),
                        resource_id: resource_id.clone(),
                        data: data.clone(),
                        timestamp: Utc::now(),
                    };
                    
                    let url = hook.url.clone();
                    let secret = hook.secret.clone();
                    
                    // Spawn delivery
                    tokio::spawn(async move {
                        let svc = WebhookService::new(); // Lightweight client creation
                        log::info!("🚀 Sending webhook {} to {}", evt.event_type, url);
                        let _ = svc.deliver_webhook(&url, &evt, Some(&secret), 3).await;
                    });
                }
            }
        }
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
