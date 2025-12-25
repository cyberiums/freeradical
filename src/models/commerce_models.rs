use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{products, orders, order_items};

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub sku: Option<String>,
    pub inventory_count: Option<i32>,
    pub is_active: bool,
    pub metadata: Option<String>, // JSON stored as string
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub sku: Option<String>,
    pub inventory_count: Option<i32>,
    pub is_active: Option<bool>,
    pub metadata: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i64,
    pub user_uuid: String,
    pub total_cents: i64,
    pub currency: String,
    pub status: String,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub metadata: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_uuid: String,
    pub total_cents: i64,
    pub currency: String,
    pub status: String,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Product))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
