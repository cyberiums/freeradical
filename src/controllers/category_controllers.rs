use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::module_models::{ModuleCategory, MutCategory};
use crate::models::{pool_handler, Model, DatabasePool};
use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

/// Create a new category for organizing modules
#[utoipa::path(
    post,
    path = "/v1/category",
    tag = "Content - Modules",
    request_body = MutCategory,
    responses(
        (status = 201, description = "Category created successfully", body = MutCategory),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_category(
    new: web::Json<MutCategory>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    ModuleCategory::create(&uuid_new, &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(uuid_new))
}

/// Update an existing category
#[utoipa::path(
    put,
    path = "/v1/category/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Category UUID to update", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    request_body = MutCategory,
    responses(
        (status = 200, description = "Category updated successfully", body = MutCategory),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Category not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_category(
    updated_category: web::Json<MutCategory>,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    ModuleCategory::update(id.clone(), &updated_category, &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(updated_category.0))
}

/// Get a single category by UUID
#[utoipa::path(
    get,
    path = "/v1/category/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Category UUID", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Category details", body = ModuleCategory),
        (status = 404, description = "Category not found")
    )
)]
pub async fn get_category(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::read_one(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

/// Delete a category
#[utoipa::path(
    delete,
    path = "/v1/category/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Category UUID to delete", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Category deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Category not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_category(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}
