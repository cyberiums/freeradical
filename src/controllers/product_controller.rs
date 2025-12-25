use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{pool_handler, MySQLPool};
use crate::models::commerce_models::{Product, NewProduct};
use crate::services::errors_service::CustomHttpError;
use crate::services::auth_service::Claims;
use crate::schema::products;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Serialize)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// List products (paginated, public)
pub async fn list_products(
    query: web::Query<PaginationQuery>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(20).min(100); // Max 100 per page
    
    let products_list = products::table
        .filter(products::is_active.eq(true))
        .limit(per_page)
        .offset(page * per_page)
        .load::<Product>(&mut conn)
        .map_err(|_| CustomHttpError::InternalServerError)?;
    
    let total = products::table
        .filter(products::is_active.eq(true))
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(|_| CustomHttpError::InternalServerError)?;
    
    let response = ProductListResponse {
        products: products_list,
        total,
        page,
        per_page,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

// Get single product (public)
pub async fn get_product(
    id: web::Path<i64>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    let product = products::table
        .find(*id)
        .filter(products::is_active.eq(true))
        .first::<Product>(&mut conn)
        .map_err(|_| CustomHttpError::NotFound)?;
    
    Ok(HttpResponse::Ok().json(product))
}

// Create product (admin only - requires authentication)
pub async fn create_product(
    product: web::Json<NewProduct>,
    pool: web::Data<MySQLPool>,
    _claim: Claims, // Authenticated user
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    diesel::insert_into(products::table)
        .values(&product.into_inner())
        .execute(&mut conn)
        .map_err(|_| CustomHttpError::InternalServerError)?;
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Product created successfully"
    })))
}

// Update product (admin only)
pub async fn update_product(
    id: web::Path<i64>,
    product: web::Json<NewProduct>,
    pool: web::Data<MySQLPool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    let updated = diesel::update(products::table.find(*id))
        .set(&product.into_inner())
        .execute(&mut conn)
        .map_err(|_| CustomHttpError::InternalServerError)?;
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound);
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product updated successfully"
    })))
}

// Delete product (admin only - soft delete)
pub async fn delete_product(
    id: web::Path<i64>,
    pool: web::Data<MySQLPool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    let updated = diesel::update(products::table.find(*id))
        .set(products::is_active.eq(false))
        .execute(&mut conn)
        .map_err(|_| CustomHttpError::InternalServerError)?;
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound);
    }
    
    Ok(HttpResponse::NoContent().finish())
}
