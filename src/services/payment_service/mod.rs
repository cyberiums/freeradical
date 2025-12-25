use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod stripe;
pub mod paypal;
pub mod square;

/// Represents a payment intent result from any provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub status: PaymentStatus,
    pub client_secret: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Succeeded,
    Failed,
    Canceled,
}

/// Request to create a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub amount_cents: i64,
    pub currency: String,
    pub metadata: HashMap<String, String>,
}

/// Trait that all payment providers must implement
#[async_trait]
pub trait PaymentHandler: Send + Sync {
    /// Unique identifier for this payment handler (e.g., "stripe", "paypal")
    fn provider_name(&self) -> &str;
    
    /// Create a new payment intent
    async fn create_payment_intent(
        &self,
        request: CreatePaymentIntentRequest,
    ) -> Result<PaymentIntent, String>;
    
    /// Retrieve an existing payment intent
    async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String>;
    
    /// Confirm a payment intent (complete the payment)
    async fn confirm_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String>;
    
    /// Cancel a payment intent
    async fn cancel_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent, String>;
    
    /// Verify webhook signature (for payment status updates)
    fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> Result<bool, String>;
}

/// Registry to manage multiple payment handlers
pub struct PaymentHandlerRegistry {
    handlers: HashMap<String, Box<dyn PaymentHandler>>,
}

impl PaymentHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a payment handler
    pub fn register(&mut self, handler: Box<dyn PaymentHandler>) {
        let name = handler.provider_name().to_string();
        log::info!("Registering payment handler: {}", name);
        self.handlers.insert(name, handler);
    }
    
    /// Get a payment handler by name
    pub fn get(&self, provider: &str) -> Option<&dyn PaymentHandler> {
        self.handlers.get(provider).map(|h| h.as_ref())
    }
    
    /// List all registered payment handlers
    pub fn list_handlers(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockPaymentHandler;
    
    #[async_trait]
    impl PaymentHandler for MockPaymentHandler {
        fn provider_name(&self) -> &str {
            "mock"
        }
        
        async fn create_payment_intent(
            &self,
            request: CreatePaymentIntentRequest,
        ) -> Result<PaymentIntent, String> {
            Ok(PaymentIntent {
                id: "mock_pi_123".to_string(),
                amount_cents: request.amount_cents,
                currency: request.currency,
                status: PaymentStatus::Pending,
                client_secret: Some("mock_secret".to_string()),
                metadata: request.metadata,
            })
        }
        
        async fn get_payment_intent(&self, _intent_id: &str) -> Result<PaymentIntent, String> {
            unimplemented!()
        }
        
        async fn confirm_payment_intent(&self, _intent_id: &str) -> Result<PaymentIntent, String> {
            unimplemented!()
        }
        
        async fn cancel_payment_intent(&self, _intent_id: &str) -> Result<PaymentIntent, String> {
            unimplemented!()
        }
        
        fn verify_webhook_signature(&self, _payload: &[u8], _signature: &str) -> Result<bool, String> {
            Ok(true)
        }
    }
    
    #[tokio::test]
    async fn test_payment_registry() {
        let mut registry = PaymentHandlerRegistry::new();
        registry.register(Box::new(MockPaymentHandler));
        
        assert!(registry.get("mock").is_some());
        assert_eq!(registry.list_handlers(), vec!["mock"]);
    }
}
