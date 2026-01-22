use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::db_connection;
use crate::middleware::auth_middleware::get_user_context;
use crate::controllers::tenant_controller;
use serde::Deserialize;
use utoipa::ToSchema;use diesel::prelude::*;

#[derive(Deserialize, ToSchema)]
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

#[derive(Deserialize, ToSchema)]
pub struct UpdateSiteRequest {
    pub name: Option<String>,
    pub subdomain: Option<String>,
    pub custom_domain: Option<String>,
    pub is_active: Option<bool>,
}

/// Update a site
#[utoipa::path(
    put,
    path = "/v1/sites/{id}",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Site ID", example = 1)
    ),
    request_body = UpdateSiteRequest,
    responses(
        (status = 200, description = "Site updated"),
        (status = 404, description = "Site not found"),
        (status = 403, description = "Access denied"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_site(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>,
    item: web::Json<UpdateSiteRequest>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let site_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    use crate::schema::{tenants, tenant_members};

    // 1. Verify user is a member of this tenant
    let membership = tenant_members::table
        .filter(tenant_members::tenant_id.eq(site_id))
        .filter(tenant_members::user_id.eq(user_ctx.user_id))
        .select(tenant_members::role)
        .first::<String>(&mut conn);

    match &membership {
        Ok(role) => {
            // Only owner or admin can update
            if role != "owner" && role != "admin" {
                return HttpResponse::Forbidden().json("Only owner or admin can update site");
            }
        },
        Err(_) => return HttpResponse::Forbidden().json("Access denied"),
    }

    // 2. Update the site
    // Build update query dynamically based on what fields are provided
    let mut updated = false;

    if let Some(is_active) = item.is_active {
        let res = diesel::update(tenants::table.find(site_id))
            .set(tenants::is_active.eq(Some(is_active)))
            .execute(&mut conn);
        
        match res {
            Ok(_) => {
                updated = true;
                println!("Successfully updated is_active to {} for site {}", is_active, site_id);
            },
            Err(e) => return HttpResponse::InternalServerError().json(format!("Error updating is_active: {}", e)),
        }
    }

    if let Some(ref name) = item.name {
        let res = diesel::update(tenants::table.find(site_id))
            .set(tenants::name.eq(name))
            .execute(&mut conn);
        if res.is_ok() { updated = true; }
    }

    if let Some(ref custom_domain) = item.custom_domain {
        let res = diesel::update(tenants::table.find(site_id))
            .set(tenants::custom_domain.eq(Some(custom_domain.clone())))
            .execute(&mut conn);
        if res.is_ok() { updated = true; }
    }

    if updated {
        HttpResponse::Ok().json(serde_json::json!({ "success": true, "message": "Site updated successfully" }))
    } else {
        HttpResponse::Ok().json(serde_json::json!({ "success": true, "message": "No changes made" }))
    }
}
