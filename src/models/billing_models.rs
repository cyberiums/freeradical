use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::{billing_plans, billing_subscriptions, billing_invoices, billing_payments, tenants};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = billing_plans)]
pub struct BillingPlan {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price_cents: i32,
    pub billing_interval: String,
    pub currency: String,
    pub limits: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = billing_plans)]
pub struct NewBillingPlan {
    pub name: String,
    pub code: String,
    pub price_cents: i32,
    pub billing_interval: String, // "month", "year"
    pub currency: String,
    pub limits: Option<serde_json::Value>,
}

use crate::models::tenant_models::Tenant;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Tenant))]
#[diesel(belongs_to(BillingPlan, foreign_key = plan_id))]
#[diesel(table_name = billing_subscriptions)]
pub struct Subscription {
    pub id: i32,
    pub tenant_id: i32,
    pub plan_id: i32,
    pub status: String, // "active", "past_due", "canceled"
    pub current_period_start: NaiveDateTime,
    pub current_period_end: NaiveDateTime,
    pub cancel_at_period_end: Option<bool>,
    pub canceled_at: Option<NaiveDateTime>,
    pub provider_subscription_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = billing_subscriptions)]
pub struct NewSubscription {
    pub tenant_id: i32,
    pub plan_id: i32,
    pub status: String,
    pub current_period_start: NaiveDateTime,
    pub current_period_end: NaiveDateTime,
    pub provider_subscription_id: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Subscription))]
#[diesel(table_name = billing_invoices)]
pub struct Invoice {
    pub id: i32,
    pub subscription_id: i32,
    pub amount_cents: i32,
    pub status: String, // "draft", "open", "paid", "void", "uncollectible"
    pub due_date: NaiveDateTime,
    pub paid_at: Option<NaiveDateTime>,
    pub line_items: Option<serde_json::Value>,
    pub invoice_number: Option<String>,
    pub pdf_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = billing_invoices)]
pub struct NewInvoice {
    pub subscription_id: i32,
    pub amount_cents: i32,
    pub status: String,
    pub due_date: NaiveDateTime,
    pub line_items: Option<serde_json::Value>,
    pub invoice_number: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = billing_invoices)]
pub struct UpdateInvoice {
    pub status: Option<String>,
    pub paid_at: Option<NaiveDateTime>,
    pub pdf_url: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = billing_payments)]
pub struct Payment {
    pub id: i32,
    pub invoice_id: i32,
    pub amount_cents: i32,
    pub provider_transaction_id: Option<String>,
    pub status: String, // "succeeded", "pending", "failed"
    pub payment_method: Option<String>,
    pub payment_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = billing_payments)]
pub struct NewPayment {
    pub invoice_id: i32,
    pub amount_cents: i32,
    pub provider_transaction_id: Option<String>,
    pub status: String,
    pub payment_method: Option<String>,
    pub payment_date: Option<NaiveDateTime>,
}
