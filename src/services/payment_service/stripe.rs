use super::{PaymentHandler, PaymentIntent, CreatePaymentIntentRequest, PaymentStatus};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct StripePaymentHandler {
    api_key: String,
    client: Client,
}

#[derive(Serialize)]
struct StripeCreateIntentRequest {
    amount: i64,
    currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct StripePaymentIntent {
    id: String,
    amount: i64,
    currency: String,
    status: String,
    client_secret: Option<String>,
    #[serde(default)]
    metadata: HashMap<String, String>,
}

impl StripePaymentHandler {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
    
    fn map_status(status: &str) -> PaymentStatus {
        match status {
            "requires_payment_method" | "requires_confirmation" | "requires_action" => PaymentStatus::Pending,
            "processing" => PaymentStatus::Processing,
            "succeeded" => PaymentStatus::Succeeded,
            "canceled" => PaymentStatus::Canceled,
            _ => PaymentStatus::Failed,
        }
    }
}

#[async_trait]
impl PaymentHandler for StripePaymentHandler {
    fn provider_name(&self) -> &str {
        "stripe"
    }
    
    async fn create_payment_intent(
        &self,
        request: CreatePaymentIntentRequest,
    ) -> Result<PaymentIntent, String> {
        let url = "https://api.stripe.com/v1/payment_intents";
        
        let stripe_request = StripeCreateIntentRequest {
            amount: request.amount_cents,
            currency: request.currency.to_lowercase(),
            metadata: if request.metadata.is_empty() {
                None
            } else {
                Some(request.metadata.clone())
            },
        };
        
        let response = self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .form(&stripe_request)
            .send()
            .await
            .map_err(|e| format!("Stripe API error: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Stripe API failed: {}", error_text));
        }
        
        let stripe_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe response: {}", e))?;
        
        Ok(PaymentIntent {
            id: stripe_intent.id,
            amount_cents: stripe_intent.amount,
            currency: stripe_intent.currency,
            status: Self::map_status(&stripe_intent.status),
            client_secret: stripe_intent.client_secret,
            metadata: stripe_intent.metadata,
        })
    }
    
    async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let url = format!("https://api.stripe.com/v1/payment_intents/{}", intent_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| format!("Stripe API error: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Stripe API failed with status: {}", response.status()));
        }
        
        let stripe_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe response: {}", e))?;
        
        Ok(PaymentIntent {
            id: stripe_intent.id,
            amount_cents: stripe_intent.amount,
            currency: stripe_intent.currency,
            status: Self::map_status(&stripe_intent.status),
            client_secret: stripe_intent.client_secret,
            metadata: stripe_intent.metadata,
        })
    }
    
    async fn confirm_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let url = format!("https://api.stripe.com/v1/payment_intents/{}/confirm", intent_id);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| format!("Stripe API error: {}", e))?;
        
        let stripe_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe response: {}", e))?;
        
        Ok(PaymentIntent {
            id: stripe_intent.id,
            amount_cents: stripe_intent.amount,
            currency: stripe_intent.currency,
            status: Self::map_status(&stripe_intent.status),
            client_secret: stripe_intent.client_secret,
            metadata: stripe_intent.metadata,
        })
    }
    
    async fn cancel_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let url = format!("https://api.stripe.com/v1/payment_intents/{}/cancel", intent_id);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| format!("Stripe API error: {}", e))?;
        
        let stripe_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe response: {}", e))?;
        
        Ok(PaymentIntent {
            id: stripe_intent.id,
            amount_cents: stripe_intent.amount,
            currency: stripe_intent.currency,
            status: Self::map_status(&stripe_intent.status),
            client_secret: stripe_intent.client_secret,
            metadata: stripe_intent.metadata,
        })
    }
    
    fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> Result<bool, String> {
        // Stripe webhook verification using HMAC-SHA256
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET")
            .map_err(|_| "STRIPE_WEBHOOK_SECRET not set".to_string())?;
        
        // Parse signature header (format: "t=timestamp,v1=signature")
        let parts: HashMap<&str, &str> = signature
            .split(',')
            .filter_map(|part| {
                let mut split = part.split('=');
                Some((split.next()?, split.next()?))
            })
            .collect();
        
        let timestamp = parts.get("t").ok_or("Missing timestamp")?;
        let sig = parts.get("v1").ok_or("Missing v1 signature")?;
        
        // Construct signed payload
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
        
        // Compute HMAC
        let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
            .map_err(|e| format!("Invalid key: {}", e))?;
        mac.update(signed_payload.as_bytes());
        
        let expected = hex::encode(mac.finalize().into_bytes());
        
        Ok(&expected == sig)
    }
}
