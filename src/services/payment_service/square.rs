use super::{PaymentHandler, PaymentIntent, CreatePaymentIntentRequest, PaymentStatus};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SquarePaymentHandler {
    access_token: String,
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct SquareCreatePaymentRequest {
    source_id: String,
    idempotency_key: String,
    amount_money: SquareMoney,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SquareMoney {
    amount: i64,
    currency: String,
}

#[derive(Deserialize)]
struct SquarePaymentResponse {
    payment: SquarePayment,
}

#[derive(Deserialize)]
struct SquarePayment {
    id: String,
    status: String,
    amount_money: SquareMoney,
    #[serde(default)]
    reference_id: Option<String>,
}

impl SquarePaymentHandler {
    pub fn new(access_token: String, sandbox: bool) -> Self {
        let base_url = if sandbox {
            "https://connect.squareupsandbox.com".to_string()
        } else {
            "https://connect.squareup.com".to_string()
        };
        
        Self {
            access_token,
            base_url,
            client: Client::new(),
        }
    }
    
    fn map_status(status: &str) -> PaymentStatus {
        match status {
            "PENDING" | "APPROVED" => PaymentStatus::Pending,
            "COMPLETED" => PaymentStatus::Succeeded,
            "CANCELED" | "FAILED" => PaymentStatus::Canceled,
            _ => PaymentStatus::Failed,
        }
    }
    
    fn generate_idempotency_key() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

#[async_trait]
impl PaymentHandler for SquarePaymentHandler {
    fn provider_name(&self) -> &str {
        "square"
    }
    
    async fn create_payment_intent(
        &self,
        request: CreatePaymentIntentRequest,
    ) -> Result<PaymentIntent, String> {
        let url = format!("{}/v2/payments", self.base_url);
        
        // Square requires a source_id (payment method nonce from frontend)
        // For simplicity, we'll use a placeholder that should be provided in metadata
        let source_id = request.metadata
            .get("source_id")
            .cloned()
            .ok_or("Square requires 'source_id' in metadata")?;
        
        let reference_id = request.metadata.get("order_id").cloned();
        
        let square_request = SquareCreatePaymentRequest {
            source_id,
            idempotency_key: Self::generate_idempotency_key(),
            amount_money: SquareMoney {
                amount: request.amount_cents,
                currency: request.currency.to_uppercase(),
            },
            reference_id,
        };
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .header("Content-Type", "application/json")
            .header("Square-Version", "2023-12-13")
            .json(&square_request)
            .send()
            .await
            .map_err(|e| format!("Square API error: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Square API failed: {}", error_text));
        }
        
        let square_response: SquarePaymentResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Square response: {}", e))?;
        
        Ok(PaymentIntent {
            id: square_response.payment.id,
            amount_cents: square_response.payment.amount_money.amount,
            currency: square_response.payment.amount_money.currency,
            status: Self::map_status(&square_response.payment.status),
            client_secret: None, // Square handles client-side differently
            metadata: request.metadata,
        })
    }
    
    async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let url = format!("{}/v2/payments/{}", self.base_url, intent_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.access_token)
            .header("Square-Version", "2023-12-13")
            .send()
            .await
            .map_err(|e| format!("Square API error: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Square API failed with status: {}", response.status()));
        }
        
        let square_response: SquarePaymentResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Square response: {}", e))?;
        
        Ok(PaymentIntent {
            id: square_response.payment.id,
            amount_cents: square_response.payment.amount_money.amount,
            currency: square_response.payment.amount_money.currency,
            status: Self::map_status(&square_response.payment.status),
            client_secret: None,
            metadata: HashMap::new(),
        })
    }
    
    async fn confirm_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        // Square payments are automatically completed when created
        // Use the complete endpoint for manual completion
        let url = format!("{}/v2/payments/{}/complete", self.base_url, intent_id);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .header("Square-Version", "2023-12-13")
            .send()
            .await
            .map_err(|e| format!("Square API error: {}", e))?;
        
        let square_response: SquarePaymentResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Square response: {}", e))?;
        
        Ok(PaymentIntent {
            id: square_response.payment.id,
            amount_cents: square_response.payment.amount_money.amount,
            currency: square_response.payment.amount_money.currency,
            status: Self::map_status(&square_response.payment.status),
            client_secret: None,
            metadata: HashMap::new(),
        })
    }
    
    async fn cancel_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let url = format!("{}/v2/payments/{}/cancel", self.base_url, intent_id);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .header("Square-Version", "2023-12-13")
            .send()
            .await
            .map_err(|e| format!("Square API error: {}", e))?;
        
        let square_response: SquarePaymentResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Square response: {}", e))?;
        
        Ok(PaymentIntent {
            id: square_response.payment.id,
            amount_cents: square_response.payment.amount_money.amount,
            currency: square_response.payment.amount_money.currency,
            status: Self::map_status(&square_response.payment.status),
            client_secret: None,
            metadata: HashMap::new(),
        })
    }
    
    fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> Result<bool, String> {
        // Square webhook verification uses HMAC-SHA256
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let webhook_signature_key = std::env::var("SQUARE_WEBHOOK_SIGNATURE_KEY")
            .map_err(|_| "SQUARE_WEBHOOK_SIGNATURE_KEY not set".to_string())?;
        
        // Construct notification URL + body
        let notification_url = std::env::var("SQUARE_WEBHOOK_URL")
            .unwrap_or_else(|_| "https://example.com/webhooks/square".to_string());
        
        let string_to_sign = format!("{}{}", notification_url, String::from_utf8_lossy(payload));
        
        // Compute HMAC
        let mut mac = HmacSha256::new_from_slice(webhook_signature_key.as_bytes())
            .map_err(|e| format!("Invalid key: {}", e))?;
        mac.update(string_to_sign.as_bytes());
        
        let expected = base64::encode(mac.finalize().into_bytes());
        
        Ok(&expected == signature)
    }
}
