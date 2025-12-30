use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingEvent {
    pub tenant_id: i32,
    pub event_type: String, // "invoice_paid", "payment_failed", "subscription_renewed"
    pub amount_cents: i32,
    pub plan_name: String,
    pub invoice_number: String,
}
