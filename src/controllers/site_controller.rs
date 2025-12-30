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

/// List user's sites
#[utoipa::path(
    get,
    path = "/v1/sites",
    tag = "Internal - System",
    responses(
        (status = 200, description = "List of sites (alias for tenants)"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_sites(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
    tenant_controller::list_my_tenants(req, pool).await
}

/// Create a new site
#[utoipa::path(
    post,
    path = "/v1/sites",
    tag = "Internal - System",
    request_body(content = String, description = "Site creation request (alias for tenant creation)"),
    responses(
        (status = 200, description = "Site created"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_site(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<tenant_controller::NewTenantRequest>
) -> impl Responder {
    tenant_controller::create_tenant(req, pool, item).await
}

/// Validate custom domain CNAME
#[utoipa::path(
    post,
    path = "/v1/sites/validate-cname",
    tag = "Internal - System",
    request_body = ValidateCnameRequest,
    responses(
        (status = 200, description = "Domain validation result"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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

/// Get site details
#[utoipa::path(
    get,
    path = "/v1/sites/{id}",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Site ID", example = 1)
    ),
    responses(
        (status = 200, description = "Site details"),
        (status = 404, description = "Site not found"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_site(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    tenant_controller::get_tenant_details(req, pool, path).await
}
