use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{
    inventory_models::{NewProductVariant, ProductVariant},
    inventory_models::InventoryAuditLog,
    inventory_models::NewInventoryAuditLog,
    DbPool,
};
use crate::schema::product_variants;
use crate::schema::inventory_audit_log;
use crate::services::errors_service::CustomHttpError;

// Import DbPool from parent module where it's defined
// type DbPool = crate::models::DbPool; // This line is removed as DbPool is now imported directly

#[derive(Serialize)]
struct GenericHttpResponse {
    message: String,
}


/// Request body for creating a product variant
#[derive(Debug, Deserialize)]
pub struct CreateVariantRequest {
    pub variant_name: Option<String>,
    pub variant_sku: Option<String>,
    pub price: Option<BigDecimal>,
    pub compare_at_price: Option<BigDecimal>,
    pub weight: Option<BigDecimal>,
    pub stock_quantity: i32,
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
}

/// Request body for updating stock
#[derive(Debug, Deserialize)]
pub struct UpdateStockRequest {
    pub quantity_change: i32,
    pub reason: Option<String>,
}

/// Get all variants for a product
pub async fn get_product_variants(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let product_id = product_id.into_inner();
    
    let variants = match web::block(move || -> Result<Vec<ProductVariant>, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        product_variants::table
            .select((product_variants::id, product_variants::uuid, product_variants::product_id, product_variants::sku, product_variants::variant_name, product_variants::price, product_variants::stock_quantity, product_variants::weight, product_variants::image_url, product_variants::is_active, product_variants::created_at, product_variants::updated_at))
            .filter(product_variants::product_id.eq(product_id))
            .filter(product_variants::is_active.eq(true))
            .load::<ProductVariant>(&mut conn)
    })
    .await {
        Ok(Ok(value)) => value,
        Ok(Err(e)) => return Err(CustomHttpError::InternalServerError(e.to_string())),
        Err(e) => return Err(CustomHttpError::InternalServerError(e.to_string())),
    };

    Ok(HttpResponse::Ok().json(variants))
}

/// Create a new product variant
pub async fn create_variant(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateVariantRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let payload = payload.into_inner();
    
    let new_variant = NewProductVariant {
        uuid: uuid::Uuid::new_v4().to_string(),
        product_id: 0, // Will be set from URL path parameter
        sku: payload.variant_sku,
        variant_name: payload.variant_name.unwrap_or_else(|| "Default".to_string()),
        price: payload.price,
        stock_quantity: Some(payload.stock_quantity),
        weight: payload.weight,
        image_url: payload.image_url,
        is_active: Some(true),
    };

    let variant = match web::block(move || -> Result<ProductVariant, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        diesel::insert_into(product_variants::table)
            .values(&new_variant)
            .execute(&mut conn)?;
        
        // Get the last inserted variant
        product_variants::table
            .select((product_variants::id, product_variants::uuid, product_variants::product_id, product_variants::sku, product_variants::variant_name, product_variants::price, product_variants::stock_quantity, product_variants::weight, product_variants::image_url, product_variants::is_active, product_variants::created_at, product_variants::updated_at))
            .order(product_variants::id.desc())
            .first::<ProductVariant>(&mut conn)
    })
    .await {
        Ok(Ok(value)) => value,
        Ok(Err(e)) => return Err(CustomHttpError::InternalServerError(format!("DB error: {}", e))),
        Err(e) => return Err(CustomHttpError::InternalServerError(format!("Block error: {}", e))),
    };

    Ok(HttpResponse::Created().json(variant))
}

/// Update variant stock and log the change
pub async fn update_variant_stock(
    pool: web::Data<DbPool>,
    variant_id: web::Path<i32>,
    payload: web::Json<UpdateStockRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let variant_id = variant_id.into_inner();
    let quantity_change = payload.quantity_change;
    let reason = payload.reason.clone();

    let variant = match web::block(move || -> Result<ProductVariant, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        
        // Get current variant
        let current: ProductVariant = product_variants::table
            .select((product_variants::id, product_variants::uuid, product_variants::product_id, product_variants::sku, product_variants::variant_name, product_variants::price, product_variants::stock_quantity, product_variants::weight, product_variants::image_url, product_variants::is_active, product_variants::created_at, product_variants::updated_at))
            .find(variant_id)
            .first(&mut conn)?;
        
        let old_quantity = current.stock_quantity.unwrap_or(0);
        let new_quantity = old_quantity + quantity_change;
        
        // Update stock
        diesel::update(product_variants::table.find(variant_id))
            .set(product_variants::stock_quantity.eq(new_quantity))
            .execute(&mut conn)?;
        
        // TODO: Re-enable when inventory_audit_log table is added
        // Create audit log
        let audit = NewInventoryAuditLog {
            product_id: Some(current.product_id),
            variant_id: Some(variant_id),
            user_id: None, // Auth integration ready - uncomment when audit log enabled
            order_id: None,
            change_type: if quantity_change > 0 {
                "restock".to_string()
            } else {
                "adjustment".to_string()
            },
            quantity_before: old_quantity,
            quantity_after: new_quantity,
            quantity_change,
            reason,
        };
        
        diesel::insert_into(inventory_audit_log::table)
            .values(&audit)
            .execute(&mut conn)?;
        
        // Return updated variant
        product_variants::table
            .select((product_variants::id, product_variants::uuid, product_variants::product_id, product_variants::sku, product_variants::variant_name, product_variants::price, product_variants::stock_quantity, product_variants::weight, product_variants::image_url, product_variants::is_active, product_variants::created_at, product_variants::updated_at))
            .find(variant_id)
            .first::<ProductVariant>(&mut conn)
    })
    .await {
        Ok(Ok(value)) => value,
        Ok(Err(e)) => return Err(CustomHttpError::InternalServerError(e.to_string())),
        Err(e) => return Err(CustomHttpError::InternalServerError(e.to_string())),
    };

    Ok(HttpResponse::Ok().json(variant))
}


/// Get inventory audit log for a product
pub async fn get_inventory_audit_log(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let product_id = product_id.into_inner();
    
    let logs = match web::block(move || -> Result<Vec<InventoryAuditLog>, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        inventory_audit_log::table
            .filter(inventory_audit_log::product_id.eq(product_id))
            .order(inventory_audit_log::created_at.desc())
            .limit(100)
            .load::<InventoryAuditLog>(&mut conn)
    })
    .await {
        Ok(Ok(value)) => value,
        Ok(Err(e)) => return Err(CustomHttpError::InternalServerError(e.to_string())),
        Err(e) => return Err(CustomHttpError::InternalServerError(e.to_string())),
    };

    Ok(HttpResponse::Ok().json(logs))
}


/// Delete a variant (soft delete by setting is_active = false)
pub async fn delete_variant(
    pool: web::Data<DbPool>,
    variant_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let variant_id = variant_id.into_inner();
    
    match web::block(move || -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string())
            ))?;
        diesel::update(product_variants::table.find(variant_id))
            .set(product_variants::is_active.eq(false))
            .execute(&mut conn)
    })
    .await {
        Ok(Ok(_)) => {},
        Ok(Err(e)) => return Err(CustomHttpError::InternalServerError(e.to_string())),
        Err(e) => return Err(CustomHttpError::InternalServerError(e.to_string())),
    }

    Ok(HttpResponse::Ok().json(GenericHttpResponse {
        message: "Variant deleted successfully".to_string(),
    }))
}
