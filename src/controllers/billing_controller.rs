use actix_web::{web, HttpResponse, Responder, get, post};
use serde::Deserialize;
use utoipa::ToSchema;
use crate::services::billing_service::BillingService;
use crate::models::db_connection::DatabasePool;
use crate::services::auth_service::Claims;
use crate::models::tenant_models::Tenant;
use crate::models::billing_models::{BillingPlan, NewBillingPlan, Subscription, Invoice};
use crate::services::payment_service::{PaymentHandlerRegistry, CreatePaymentIntentRequest};
use crate::schema::{billing_plans, billing_subscriptions, billing_invoices};
use diesel::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct SubscribeRequest {
    pub plan_code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ConfirmPaymentRequest {
    pub subscription_id: i32,
    pub payment_intent_id: Option<String>,
    pub payment_method_id: Option<String>,
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
    tag = "Internal - Billing",
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
    tag = "Internal - Billing",
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
    payment_registry: web::Data<PaymentHandlerRegistry>,
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
        Ok(sub) => {
            // 5. Create Stripe Payment Intent
            // Fetch the invoice created by subscribe_tenant
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            
            let invoice = billing_invoices::table
                .filter(billing_invoices::subscription_id.eq(sub.id))
                .order(billing_invoices::created_at.desc())
                .first::<Invoice>(&mut conn);
                
            let client_secret = if let Ok(inv) = invoice {
                if let Some(stripe) = payment_registry.get("stripe") {
                    let mut metadata = std::collections::HashMap::new();
                    metadata.insert("subscription_id".to_string(), sub.id.to_string());
                    
                    let req = CreatePaymentIntentRequest {
                        amount_cents: inv.amount_cents.into(),
                        currency: "usd".to_string(), // Lowercase currency is safer for Stripe
                        metadata,
                    };
                    
                    match stripe.create_payment_intent(req).await {
                        Ok(intent) => intent.client_secret,
                        Err(e) => return HttpResponse::BadRequest().json(format!("Stripe Error: {}", e)),
                    }
                } else {
                    return HttpResponse::InternalServerError().json("Payment provider 'stripe' not configured");
                }
            } else {
                None
            };
            
            HttpResponse::Ok().json(serde_json::json!({
                "subscription": sub,
                "client_secret": client_secret
            }))
        },
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

/// Cancel current subscription
#[utoipa::path(
    post,
    path = "/api/billing/cancel",
    tag = "Internal - Billing",
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

/// Confirm payment for subscription
#[utoipa::path(
    post,
    path = "/api/billing/confirm-payment",
    tag = "Internal - Billing",
    request_body = ConfirmPaymentRequest,
    responses(
        (status = 200, description = "Payment confirmed, subscription active"),
        (status = 400, description = "Payment failed")
    ),
    security((
        "bearer_auth" = []
    ))
)]
#[post("/api/billing/confirm-payment")]
pub async fn confirm_payment(
    _req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    payment_registry: web::Data<PaymentHandlerRegistry>,
    item: web::Json<ConfirmPaymentRequest>,
    claims: Claims,
    cache: web::Data<crate::services::cache_service_v2::CacheServiceV2>,
) -> impl Responder {
    // 1. Verify Payment Intent with Stripe if provided
    if let Some(payment_intent_id) = &item.payment_intent_id {
        if let Some(stripe) = payment_registry.get("stripe") {
             match stripe.get_payment_intent(payment_intent_id).await {
                Ok(intent) => {
                    // Check status enum or string? implementation returns custom struct with enum status
                    // checking simplistic string mapping or if status is Succeeded
                    // The PaymentHandler trait returns PaymentIntent struct which has 'status' field enum
                    use crate::services::payment_service::PaymentStatus;
                    match intent.status {
                        PaymentStatus::Succeeded => {
                             // Proceed to activate
                        },
                        _ => return HttpResponse::BadRequest().json("Payment not successful"),
                    }
                },
                Err(e) => return HttpResponse::BadRequest().json(format!("Stripe verification failed: {}", e)),
             }
        }
    }

    match BillingService::confirm_subscription_payment(&pool, item.subscription_id) {
        Ok(sub) => {
            // Invalidate Dashboard Settings Cache so UI updates immediately
            let cache_key = format!("page:{}:/dashboard/settings:html", sub.tenant_id);
            let _ = cache.delete(&cache_key).await;
            
            HttpResponse::Ok().json(sub)
        },
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[get("/api/config/stripe")]
pub async fn get_stripe_config() -> impl Responder {
    let key = std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_else(|_| "".to_string());
    HttpResponse::Ok().json(serde_json::json!({
        "publishableKey": key
    }))
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
    _req: actix_web::HttpRequest,
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
    _req: actix_web::HttpRequest,
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
    tag = "Internal - Billing",
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
    println!("TRACE: get_my_subscription called for user {}", claims.sub);
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid user ID in token"),
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    println!("TRACE: Resolving tenant for user_id {}", user_id);
    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => {
            println!("TRACE: Found tenant_id {}", tid);
            tid
        },
        Ok(None) => return HttpResponse::Forbidden().json("User does not belong to any tenant"),
        Err(e) => {
            println!("TRACE: Error getting tenant: {}", e);
            return HttpResponse::InternalServerError().json(format!("Error determining tenant: {}", e))
        },
    };

     match BillingService::get_subscription_details(&pool, tenant_id) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            println!("TRACE: BillingService Error: {}", e);
            HttpResponse::InternalServerError().json(e)
        },
     }
}

/// View Invoice (Printable)
#[utoipa::path(
    get,
    path = "/api/billing/invoices/{id}/view",
    tag = "Internal - Billing",
    params(
        ("id" = i32, Path, description = "Invoice ID")
    ),
    responses(
        (status = 200, description = "Invoice HTML view"),
        (status = 404, description = "Invoice not found")
    ),
    security((
        "bearer_auth" = []
    ))
)]
#[get("/api/billing/invoices/{id}/view")]
pub async fn get_invoice_view(
    req: actix_web::HttpRequest,
    pool: web::Data<DatabasePool>,
    hb: web::Data<std::sync::Mutex<handlebars::Handlebars<'_>>>,
    path: web::Path<i32>,
    claims: Claims,
) -> impl Responder {
    let invoice_id = path.into_inner();
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user ID"),
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB Error"),
    };

    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => tid,
        _ => return HttpResponse::Forbidden().body("User not in tenant"),
    };

    match BillingService::get_invoice_details(&pool, invoice_id, tenant_id) {
        Ok(data) => {
             let hb = hb.lock().unwrap();
             let body = hb.render("billing/invoice", &data).unwrap_or_else(|e| format!("Render error: {}", e));
             HttpResponse::Ok().content_type("text/html").body(body)
        },
        Err(e) => HttpResponse::Forbidden().body(e),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_plans);
    cfg.service(subscribe);
    cfg.service(cancel_subscription);
    cfg.service(confirm_payment);
    cfg.service(get_stripe_config);
    // Views
    cfg.service(get_billing_view);
    cfg.service(get_plans_view);
    cfg.service(get_my_subscription);
    cfg.service(get_invoice_view);
}
