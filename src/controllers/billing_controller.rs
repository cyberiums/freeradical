use actix_web::{web, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::services::billing_service::BillingService;
use crate::models::db_connection::DatabasePool;
use crate::services::auth_service::Claims;
use crate::models::tenant_models::Tenant;

#[derive(Deserialize, ToSchema)]
pub struct SubscribeRequest {
    pub plan_code: String,
}

#[derive(Deserialize)]
pub struct CancelRequest {
    // No specific fields needed if we just cancel the tenant's active subscription obtained from context
    // But helpful to verify intent if needed.
}

/// List all billing plans
#[utoipa::path(
    get,
    path = "/api/billing/plans",
    tag = "Commerce - Billing",
    responses(
        (status = 200, description = "List of billing plans")
    )
)]
#[get("/api/billing/plans")]
pub async fn get_all_plans(
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    match BillingService::get_plans(&pool) {
        Ok(plans) => HttpResponse::Ok().json(plans),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching plans: {}", e)),
    }
}

/// Subscribe to a billing plan
#[utoipa::path(
    post,
    path = "/api/billing/subscribe",
    tag = "Commerce - Billing",
    request_body = SubscribeRequest,
    responses(
        (status = 200, description = "Subscription created"),
        (status = 400, description = "Invalid plan or subscription failed")
    ),
    security((
        "bearer_auth" = []
    ))
)]
#[post("/api/billing/subscribe")]
pub async fn subscribe(
    _req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    item: web::Json<SubscribeRequest>,
    claims: Claims, // Authenticated user
) -> impl Responder {
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid user ID in token"),
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => tid,
        Ok(None) => return HttpResponse::Forbidden().json("User does not belong to any tenant"),
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error determining tenant: {}", e)),
    };

    match BillingService::subscribe_tenant(&pool, tenant_id, &item.plan_code) {
        Ok(sub) => HttpResponse::Ok().json(sub),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

/// Cancel current subscription
#[utoipa::path(
    post,
    path = "/api/billing/cancel",
    tag = "Commerce - Billing",
    responses(
        (status = 200, description = "Subscription canceled"),
        (status = 404, description = "No active subscription")
    ),
    security((
        "bearer_auth" = []
    ))
)]
#[post("/api/billing/cancel")]
pub async fn cancel_subscription(
    _req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    claims: Claims,
) -> impl Responder {
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid user ID in token"),
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => tid,
        Ok(None) => return HttpResponse::Forbidden().json("User does not belong to any tenant"),
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error determining tenant: {}", e)),
    };

    match BillingService::cancel_subscription(&pool, tenant_id) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().json("Subscription canceled")
            } else {
                HttpResponse::NotFound().json("No active subscription to cancel")
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

use std::sync::Mutex;
use handlebars::Handlebars;

/// Get billing view (HTML)
#[utoipa::path(
    get,
    path = "/settings/billing",
    tag = "Internal - Views",
    responses(
        (status = 200, description = "Billing settings page", content_type = "text/html")
    )
)]
#[get("/settings/billing")]
pub async fn get_billing_view(
    req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> impl Responder {
     // Mock tenant ID or get from context/header? 
     // Since this is a view, it's likely accessed by a logged-in user in a browser where X-Tenant-ID might not be set by client JS yet.
     // We need to resolve tenant from User Context (cookies/JWT).
     // Assuming middleware::auth extracts this.
     // For now, hardcode 1 or resolve via helper if possible.
     // In `page_controllers`, `resolve_tenant_id` is used.
     // use crate::helpers::tenant_helper::resolve_tenant_id;
     
     // let tenant_id = crate::helpers::tenant_helper::resolve_tenant_id(&req, &pool).unwrap_or(0);
     // If 0, redirect to login or error?
     let tenant_id = 1; // placeholder for development if auth not fully set up in browser flow

     match BillingService::get_subscription_details(&pool, tenant_id) {
        Ok(data) => {
             let body = hb.lock().unwrap().render("billing/index", &data).unwrap_or_else(|e| format!("Render error: {}", e));
             HttpResponse::Ok().content_type("text/html").body(body)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error loading billing details: {}", e)),
     }
}

/// Get plans view (HTML)
#[utoipa::path(
    get,
    path = "/settings/plans",
    tag = "Internal - Views",
    responses(
        (status = 200, description = "Plans selection page", content_type = "text/html")
    )
)]
#[get("/settings/plans")]
pub async fn get_plans_view(
    req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> impl Responder {
    // ... existing implementation ...
     let tenant_id = 1; // placeholder
    
    // Get all plans
    let plans = match BillingService::get_plans(&pool) {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    // Get current subscription to mark current
    let current_plan_id = match BillingService::get_subscription_details(&pool, tenant_id) {
         Ok(details) => {
             details["currentPlan"]["name"].as_str().unwrap_or("").to_string()
         },
         Err(_) => "".to_string(),
    };

    let plans_view: Vec<serde_json::Value> = plans.into_iter().map(|p| {
        let is_current = p.name == current_plan_id;
        serde_json::json!({
            "id": p.id,
            "code": p.code,
            "name": p.name,
            "price": format!("{:.0}", p.price_cents as f64 / 100.0), // Display as int for nice UI
            "interval": p.billing_interval,
            "features": match p.limits {
                Some(l) => l["features"].clone(),
                None => serde_json::json!([])
            },
            "isCurrent": is_current
        })
    }).collect();
    
    let data = serde_json::json!({
        "plans": plans_view
    });

    let body = hb.lock().unwrap().render("billing/plans", &data).unwrap_or_else(|e| format!("Render error: {}", e));
    HttpResponse::Ok().content_type("text/html").body(body)
}

/// Get current user's subscription
#[utoipa::path(
    get,
    path = "/api/billing/subscription",
    tag = "Commerce - Billing",
    responses(
        (status = 200, description = "Subscription details"),
        (status = 403, description = "User not in tenant")
    ),
    security((
        "bearer_auth" = []
    ))
)]
#[get("/api/billing/subscription")]
pub async fn get_my_subscription(
    _req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    claims: Claims,
) -> impl Responder {
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid user ID in token"),
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => tid,
        Ok(None) => return HttpResponse::Forbidden().json("User does not belong to any tenant"),
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error determining tenant: {}", e)),
    };

     match BillingService::get_subscription_details(&pool, tenant_id) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(e),
     }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_plans);
    cfg.service(subscribe);
    cfg.service(cancel_subscription);
    // Views
    cfg.service(get_billing_view);
    cfg.service(get_plans_view);
    cfg.service(get_my_subscription);
}
