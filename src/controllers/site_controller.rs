use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::db_connection;
use crate::middleware::auth_middleware::get_user_context;
use crate::controllers::tenant_controller;
use serde::Deserialize;
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct ValidateCnameRequest {
    pub domain: String,
}

pub async fn list_sites(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
    tenant_controller::list_my_tenants(req, pool).await
}

pub async fn create_site(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<tenant_controller::NewTenantRequest>
) -> impl Responder {
    tenant_controller::create_tenant(req, pool, item).await
}

pub async fn validate_cname(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<ValidateCnameRequest>
) -> impl Responder {
     let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::tenants;

    // Check if domain exists
    let exists = tenants::table
        .filter(tenants::custom_domain.eq(&item.domain))
        .select(tenants::id)
        .first::<i32>(&mut conn)
        .optional();

    match exists {
        Ok(Some(_)) => HttpResponse::Ok().json(serde_json::json!({ "valid": false, "message": "Domain already taken" })),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({ "valid": true, "message": "Domain available" })),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error validating domain: {}", e)),
    }
}

pub async fn get_site(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    tenant_controller::get_tenant_details(req, pool, path).await
}
