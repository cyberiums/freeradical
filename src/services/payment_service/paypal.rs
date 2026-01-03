use super::{PaymentHandler, PaymentIntent, CreatePaymentIntentRequest, PaymentStatus};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct PayPalPaymentHandler {
    client_id: String,
    client_secret: String,
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct PayPalCreateOrderRequest {
    intent: String,
    purchase_units: Vec<PayPalPurchaseUnit>,
}

#[derive(Serialize)]
struct PayPalPurchaseUnit {
    amount: PayPalAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_id: Option<String>,
}

#[derive(Serialize)]
struct PayPalAmount {
    currency_code: String,
    value: String,
}

#[derive(Deserialize)]
struct PayPalOrderResponse {
    id: String,
    status: String,
    #[serde(default)]
    purchase_units: Vec<PayPalPurchaseUnitResponse>,
}

#[derive(Deserialize)]
struct PayPalPurchaseUnitResponse {
    amount: PayPalAmountResponse,
}

#[derive(Deserialize)]
struct PayPalAmountResponse {
    currency_code: String,
    value: String,
}

#[derive(Deserialize)]
struct PayPalAuthResponse {
    access_token: String,
}

impl PayPalPaymentHandler {
    pub fn new(client_id: String, client_secret: String, sandbox: bool) -> Self {
        let base_url = if sandbox {
            "https://api-m.sandbox.paypal.com".to_string()
        } else {
            "https://api-m.paypal.com".to_string()
        };
        
        Self {
            client_id,
            client_secret,
            base_url,
            client: Client::new(),
        }
    }
    
    async fn get_access_token(&self) -> Result<String, String> {
        let url = format!("{}/v1/oauth2/token", self.base_url);
        
        let response = self.client
            .post(&url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .map_err(|e| format!("PayPal auth error: {}", e))?;
        
        let auth_response: PayPalAuthResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse PayPal auth response: {}", e))?;
        
        Ok(auth_response.access_token)
    }
    
    fn map_status(status: &str) -> PaymentStatus {
        match status {
            "CREATED" | "SAVED" | "APPROVED" => PaymentStatus::Pending,
            "VOIDED" => PaymentStatus::Canceled,
            "COMPLETED" => PaymentStatus::Succeeded,
            _ => PaymentStatus::Failed,
        }
    }
}

#[async_trait]
impl PaymentHandler for PayPalPaymentHandler {
    fn provider_name(&self) -> &str {
        "paypal"
    }
    
    async fn create_payment_intent(
        &self,
        request: CreatePaymentIntentRequest,
    ) -> Result<PaymentIntent, String> {
        let token = self.get_access_token().await?;
        let url = format!("{}/v2/checkout/orders", self.base_url);
        
        // PayPal expects amount in decimal format (e.g., "50.00" for $50)
        let amount_decimal = format!("{:.2}", request.amount_cents as f64 / 100.0);
        
        let custom_id = request.metadata.get("order_id").cloned();
        
        let paypal_request = PayPalCreateOrderRequest {
            intent: "CAPTURE".to_string(),
            purchase_units: vec![PayPalPurchaseUnit {
                amount: PayPalAmount {
                    currency_code: request.currency.to_uppercase(),
                    value: amount_decimal,
                },
                custom_id,
            }],
        };
        
        let response = self.client
            .post(&url)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .json(&paypal_request)
            .send()
            .await
            .map_err(|e| format!("PayPal API error: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("PayPal API failed: {}", error_text));
        }
        
        let paypal_order: PayPalOrderResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse PayPal response: {}", e))?;
        
        Ok(PaymentIntent {
            id: paypal_order.id,
            amount_cents: request.amount_cents,
            currency: request.currency,
            status: Self::map_status(&paypal_order.status),
            client_secret: None, // PayPal doesn't use client secrets
            metadata: request.metadata,
        })
    }
    
    async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let token = self.get_access_token().await?;
        let url = format!("{}/v2/checkout/orders/{}", self.base_url, intent_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| format!("PayPal API error: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("PayPal API failed with status: {}", response.status()));
        }
        
        let paypal_order: PayPalOrderResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse PayPal response: {}", e))?;
        
        let amount_cents = if let Some(unit) = paypal_order.purchase_units.first() {
            (unit.amount.value.parse::<f64>().unwrap_or(0.0) * 100.0) as i64
        } else {
            0
        };
        
        let currency = paypal_order.purchase_units
            .first()
            .map(|u| u.amount.currency_code.clone())
            .unwrap_or_else(|| "USD".to_string());
        
        Ok(PaymentIntent {
            id: paypal_order.id,
            amount_cents,
            currency,
            status: Self::map_status(&paypal_order.status),
            client_secret: None,
            metadata: HashMap::new(),
        })
    }
    
    async fn confirm_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String> {
        let token = self.get_access_token().await?;
        let url = format!("{}/v2/checkout/orders/{}/capture", self.base_url, intent_id);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("PayPal API error: {}", e))?;
        
        let paypal_order: PayPalOrderResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse PayPal response: {}", e))?;
        
        let amount_cents = if let Some(unit) = paypal_order.purchase_units.first() {
            (unit.amount.value.parse::<f64>().unwrap_or(0.0) * 100.0) as i64
        } else {
            0
        };
        
        Ok(PaymentIntent {
            id: paypal_order.id,
            amount_cents,
            currency: paypal_order.purchase_units
                .first()
                .map(|u| u.amount.currency_code.clone())
                .unwrap_or_else(|| "USD".to_string()),
            status: Self::map_status(&paypal_order.status),
            client_secret: None,
            metadata: HashMap::new(),
        })
    }
    
    async fn cancel_payment_intent(&self, _intent_id: &str) -> Result<PaymentIntent, String> {
        // PayPal orders are automatically voided if not captured within 3 hours
        Err("PayPal orders cannot be manually canceled via API".to_string())
    }
    
    fn verify_webhook_signature(&self, _payload: &[u8], signature: &str) -> Result<bool, String> {
        // PayPal webhook verification requires additional metadata
        // For production, implement full webhook verification:
        // https://developer.paypal.com/api/rest/webhooks/
        log::warn!("PayPal webhook verification not fully implemented");
        
        // Basic check: signature should be present
        Ok(!signature.is_empty())
    }
}
