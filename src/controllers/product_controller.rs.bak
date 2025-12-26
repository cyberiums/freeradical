use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{pool_handler, DatabasePool};
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

pub async fn list_products(
    query: web::Query<PaginationQuery>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(20).min(100);
    
    let (products_list, total) = match &mut conn {
        PooledDatabaseConnection::MySQL(ref mut c) => {
            let products = products::table
                .filter(products::is_active.eq(true))
                .limit(per_page)
                .offset(page * per_page)
                .load::<Product>(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
            
            let count = products::table
                .filter(products::is_active.eq(true))
                .count()
                .get_result::<i64>(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
            
            (products, count)
        }
        PooledDatabaseConnection::Postgres(ref mut c) => {
            let products = products::table
                .filter(products::is_active.eq(true))
                .limit(per_page)
                .offset(page * per_page)
                .load::<Product>(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
            
            let count = products::table
                .filter(products::is_active.eq(true))
                .count()
                .get_result::<i64>(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
            
            (products, count)
        }
    };
    
    let response = ProductListResponse {
        products: products_list,
        total,
        page,
        per_page,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_product(
    id: web::Path<i64>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let product = match &mut conn {
        PooledDatabaseConnection::MySQL(ref mut c) => {
            products::table
                .find(*id)
                .filter(products::is_active.eq(true))
                .first::<Product>(c)
                .map_err(|_| CustomHttpError::NotFound)?
        }
        PooledDatabaseConnection::Postgres(ref mut c) => {
            products::table
                .find(*id)
                .filter(products::is_active.eq(true))
                .first::<Product>(c)
                .map_err(|_| CustomHttpError::NotFound)?
        }
    };
    
    Ok(HttpResponse::Ok().json(product))
}

pub async fn create_product(
    product: web::Json<NewProduct>,
    pool: web::Data<DatabasePool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    match &mut conn {
        PooledDatabaseConnection::MySQL(ref mut c) => {
            diesel::insert_into(products::table)
                .values(&product.into_inner())
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
        }
        PooledDatabaseConnection::Postgres(ref mut c) => {
            diesel::insert_into(products::table)
                .values(&product.into_inner())
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?;
        }
    }
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Product created successfully"
    })))
}

pub async fn update_product(
    id: web::Path<i64>,
    product: web::Json<NewProduct>,
    pool: web::Data<DatabasePool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let updated = match &mut conn {
        PooledDatabaseConnection::MySQL(ref mut c) => {
            diesel::update(products::table.find(*id))
                .set(&product.into_inner())
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?
        }
        PooledDatabaseConnection::Postgres(ref mut c) => {
            diesel::update(products::table.find(*id))
                .set(&product.into_inner())
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?
        }
    };
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound);
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product updated successfully"
    })))
}

pub async fn delete_product(
    id: web::Path<i64>,
    pool: web::Data<DatabasePool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let updated = match &mut conn {
        PooledDatabaseConnection::MySQL(ref mut c) => {
            diesel::update(products::table.find(*id))
                .set(products::is_active.eq(false))
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?
        }
        PooledDatabaseConnection::Postgres(ref mut c) => {
            diesel::update(products::table.find(*id))
                .set(products::is_active.eq(false))
                .execute(c)
                .map_err(|_| CustomHttpError::InternalServerError)?
        }
    };
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound);
    }
    
    Ok(HttpResponse::NoContent().finish())
}
