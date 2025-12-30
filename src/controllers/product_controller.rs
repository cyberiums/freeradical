use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{pool_handler, DatabasePool};
use crate::models::commerce_models::{Product, NewProduct};
use crate::services::errors_service::CustomHttpError;
use crate::schema::products;
use crate::helpers::tenant_helper::{resolve_tenant_id, get_tenant_role};
use crate::models::rbac::{has_permission, Permission};
use crate::middleware::auth_middleware::get_user_context;
use actix_web::HttpRequest;
use uuid::Uuid;

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

/// List products with pagination
#[utoipa::path(
    get,
    path = "/v1/products",
    tag = "Commerce - Products",
    params(
        ("page" = Option<i64>, Query, description = "Page number (0-indexed)", example = 0),
        ("per_page" = Option<i64>, Query, description = "Items per page (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "Paginated product list", body = ProductListResponse),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_products(
    req: HttpRequest,
    query: web::Query<PaginationQuery>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    // Optional: verify membership? Generally "List" implies ViewContent or similar. 
    // For now we just enforce filtering.

    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(20).min(100);
    
    let products_list =  products::table
                .filter(products::is_active.eq(true))
                .filter(products::tenant_id.eq(tenant_id))
                .limit(per_page)
                .offset(page * per_page)
                .load::<Product>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
            
    let total = products::table
                .filter(products::is_active.eq(true))
                .filter(products::tenant_id.eq(tenant_id))
                .count()
                .get_result::<i64>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let response = ProductListResponse {
        products: products_list,
        total,
        page,
        per_page,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Get product by ID
#[utoipa::path(
    get,
    path = "/v1/products/{id}",
    tag = "Commerce - Products",
    params(
        ("id" = i64, Path, description = "Product ID", example = 123)
    ),
    responses(
        (status = 200, description = "Product details", body = Product),
        (status = 404, description = "Product not found or inactive"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_product(
    req: HttpRequest,
    id: web::Path<i64>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;

    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    let product = 
            products::table
                .find(*id)
                .filter(products::is_active.eq(true))
                .filter(products::tenant_id.eq(tenant_id)) // Enforce tenant ownership
                .first::<Product>(&mut conn)
                .map_err(|_| CustomHttpError::NotFound("Product not found".to_string()))?
        ;
    
    Ok(HttpResponse::Ok().json(product))
}

/// Create a new product
#[utoipa::path(
    post,
    path = "/v1/products",
    tag = "Commerce - Products",
    request_body = NewProduct,
    responses(
        (status = 201, description = "Product created successfully"),
        (status = 400, description = "Invalid product data"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_product(
    req: HttpRequest,
    product: web::Json<NewProduct>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut conn)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::PublishContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }
    
    let mut new_prod = product.into_inner();
    new_prod.tenant_id = Some(tenant_id);
    new_prod.uuid = Uuid::new_v4().to_string(); // Ensure UUID is generated if not sent? Or trust input? Usually gen server side.

            diesel::insert_into(products::table)
                .values(&new_prod)
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
        
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Product created successfully"
    })))
}

/// Update an existing product
#[utoipa::path(
    put,
    path = "/v1/products/{id}",
    tag = "Commerce - Products",
    params(
        ("id" = i64, Path, description = "Product ID to update", example = 123)
    ),
    request_body = NewProduct,
    responses(
        (status = 200, description = "Product updated successfully"),
        (status = 400, description = "Invalid product data"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Product not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_product(
    req: HttpRequest,
    id: web::Path<i64>,
    product: web::Json<NewProduct>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;
    
    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut conn)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::EditContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }

    let mut updated_prod = product.into_inner();
    updated_prod.tenant_id = Some(tenant_id);

    let updated = 
            diesel::update(products::table.find(*id))
                .filter(products::tenant_id.eq(tenant_id)) // Enforce tenant check
                .set(&updated_prod)
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
        ;
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound("Not found".to_string()));
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Product updated successfully"
    })))
}

/// Delete a product (soft delete)
#[utoipa::path(
    delete,
    path = "/v1/products/{id}",
    tag = "Commerce - Products",
    params(
        ("id" = i64, Path, description = "Product ID to delete", example = 123)
    ),
    responses(
        (status = 204, description = "Product deleted successfully (soft delete)"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Product not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_product(
    req: HttpRequest,
    id: web::Path<i64>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_ctx = get_user_context(&req).ok_or(CustomHttpError::Unauthorized("Not authenticated".to_string()))?;
    let tenant_id = resolve_tenant_id(&req, &pool).map_err(|e| CustomHttpError::BadRequest(e))?;

    use crate::models::PooledDatabaseConnection;
    let mut conn = pool_handler(pool)?;
    
    // RBAC Check
    let role = get_tenant_role(tenant_id, user_ctx.user_id, &mut conn)
        .map_err(|_| CustomHttpError::Forbidden("Access denied".to_string()))?;
    
    if !has_permission(&role, Permission::DeleteContent) {
        return Err(CustomHttpError::Forbidden("Insufficient permissions".to_string()));
    }

    let updated = 
            diesel::update(products::table.find(*id))
                .filter(products::tenant_id.eq(tenant_id)) // Enforce tenant
                .set(products::is_active.eq(false))
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
        ;
    
    if updated == 0 {
        return Err(CustomHttpError::NotFound("Not found".to_string()));
    }
    
    Ok(HttpResponse::NoContent().finish())
}
