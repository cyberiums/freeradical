use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{products, orders, order_items};

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub sku: Option<String>,
    pub tenant_id: Option<i32>,
    pub inventory_count: Option<i32>,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub stock_quantity: i32,
    pub low_stock_threshold: Option<i32>,
    pub stock_status: Option<String>,
    pub track_inventory: Option<bool>,
    pub allow_backorder: Option<bool>,
    pub backorder_limit: Option<i32>,
}

#[derive(Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub sku: Option<String>,
    pub inventory_count: Option<i32>,
    pub is_active: Option<bool>,
    pub tenant_id: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i64,
    pub uuid: String,
    pub user_uuid: String,
    pub status: String,
    pub total_amount_cents: i64,
    pub tenant_id: Option<i32>,
    pub payment_status: Option<String>,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub uuid: String,
    pub user_uuid: String,
    pub status: String,
    pub total_amount_cents: i64,
    pub payment_status: Option<String>,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    // metadata excluded - can be set via separate update if needed
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Product))]
#[diesel(primary_key(id))]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub price_cents: i64,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub price_cents: i64,
}
