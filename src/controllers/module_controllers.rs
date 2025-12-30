use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::{Model, DatabasePool, pool_handler};
use crate::models::module_models::{Module, ModuleCategory, MutModule};

use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

/// Create a new content module
#[utoipa::path(
    post,
    path = "/v1/modules",
    tag = "Content - Modules",
    request_body = MutModule,
    responses(
        (status = 201, description = "Module created successfully", body = MutModule),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_module(
    new: web::Json<MutModule>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    Module::create(&uuid_new, &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(uuid_new))
}

/// List all content modules
#[utoipa::path(
    get,
    path = "/v1/modules",
    tag = "Content - Modules",
    responses(
        (status = 200, description = "List of all modules", body = Vec<Module>)
    )
)]
pub async fn get_modules(pool: web::Data<DatabasePool>) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;
    let modules = Module::read_all(&mut mysql_pool)?;

    Ok(HttpResponse::Created().json(modules))
}

/// Get a single module by UUID
#[utoipa::path(
    get,
    path = "/v1/modules/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Module UUID", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Module details", body = Module),
        (status = 404, description = "Module not found")
    )
)]
pub async fn get_module(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let module = Module::read_one(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(module))
}

/// Update an existing module
#[utoipa::path(
    put,
    path = "/v1/modules/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Module UUID to update", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    request_body = MutModule,
    responses(
        (status = 200, description = "Module updated successfully", body = MutModule),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Module not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_module(
    updated_module: web::Json<MutModule>,
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    Module::update(id.clone(), &updated_module, &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(updated_module.0))
}

/// Delete a module
#[utoipa::path(
    delete,
    path = "/v1/modules/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Module UUID to delete", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Module deleted successfully"),
        (status = 401, description = "Not authenticated"),
        (status = 404, description = "Module not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_module(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = Module::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(res))
}

/// Get modules by category
#[utoipa::path(
    get,
    path = "/v1/modules/category/{id}",
    tag = "Content - Modules",
    params(
        ("id" = String, Path, description = "Category UUID", example = "123e4567-e89b-12d3-a456-426614174000")
    ),
    responses(
        (status = 200, description = "Modules in category", body = Vec<Module>),
        (status = 404, description = "Category not found")
    )
)]
pub async fn get_module_category(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let modules = ModuleCategory::join(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Created().json(modules))
}