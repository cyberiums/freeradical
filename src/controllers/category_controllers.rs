use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::module_models::{ModuleCategory, MutCategory};
use crate::models::{pool_handler, Model, DatabasePool};
use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

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

pub async fn get_category(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::read_one(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete_category(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = ModuleCategory::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}
