use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Jsonb, Numeric};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::schema::{product_variants, inventory_audit_log};

/// Product variant for SKU-level inventory management
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, ToSchema)]
#[diesel(table_name = product_variants)]
pub struct ProductVariant {
    pub id: i32,
    pub uuid: String,
    pub product_id: i32,
    pub sku: Option<String>,
    pub variant_name: String,
    #[schema(value_type = Option<String>)]
    pub price: Option<BigDecimal>,
    pub stock_quantity: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub weight: Option<BigDecimal>,
    // attributes field excluded - JSONB type incompatible with Queryable
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// New product variant for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = product_variants)]
pub struct NewProductVariant {
    pub uuid: String,
    pub product_id: i32,
    pub sku: Option<String>,
    pub variant_name: String,
    #[schema(value_type = Option<String>)]
    pub price: Option<BigDecimal>,
    pub stock_quantity: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub weight: Option<BigDecimal>,
    // attributes excluded - JSONB type incompatible
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
}

impl Default for NewProductVariant {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            product_id: 0,
            sku: None,
            variant_name: String::new(),
            price: None,
            stock_quantity: Some(0),
            weight: None,
            image_url: None,
            is_active: Some(true),
        }
    }
}


/// Inventory audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = inventory_audit_log)]
pub struct InventoryAuditLog {
    pub id: i32,
    pub product_id: Option<i32>,
    pub variant_id: Option<i32>,
    pub user_id: Option<i32>,
    pub order_id: Option<i32>,
    pub change_type: String,
    pub quantity_before: Option<i32>,
    pub quantity_after: Option<i32>,
    pub quantity_change: Option<i32>,
    pub reason: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// New inventory audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = inventory_audit_log)]
pub struct NewInventoryAuditLog {
    pub product_id: Option<i32>,
    pub variant_id: Option<i32>,
    pub user_id: Option<i32>,
    pub order_id: Option<i32>,
    pub change_type: String,
    pub quantity_before: i32,
    pub quantity_after: i32,
    pub quantity_change: i32,
    pub reason: Option<String>,
}

impl NewInventoryAuditLog {
    pub fn new(
        product_id: i32,
        change_type: &str,
        quantity_before: i32,
        quantity_after: i32,
        reason: Option<String>,
    ) -> Self {
        Self {
            product_id: Some(product_id),
            variant_id: None,
            user_id: None,
            order_id: None,
            change_type: change_type.to_string(),
            quantity_before,
            quantity_after,
            quantity_change: quantity_after - quantity_before,
            reason,
        }
    }
}
