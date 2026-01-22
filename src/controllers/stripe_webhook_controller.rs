use actix_web::{web, HttpRequest, HttpResponse, Responder, post};
use crate::models::DatabasePool;
use crate::services::billing_service::BillingService;
use crate::services::payment_service::stripe::StripePaymentHandler;
use crate::services::payment_service::PaymentHandler;
use crate::services::cache_service_v2::CacheServiceV2;

/// Handle Stripe Webhook
#[utoipa::path(
    post,
    path = "/v1/webhooks/stripe",
    tag = "Commerce - Payments",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Event processed or ignored"),
        (status = 400, description = "Invalid signature or JSON"),
        (status = 500, description = "Internal processing error")
    )
)]
#[post("/webhooks/stripe")]
pub async fn handle_webhook(
    req: HttpRequest,
    body: web::Bytes,
    pool: web::Data<DatabasePool>,
    cache: web::Data<CacheServiceV2>,
) -> impl Responder {
    // 1. Get header
    let signature = match req.headers().get("Stripe-Signature") {
        Some(v) => v.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("Missing Stripe-Signature"),
    };

    // 2. Initialize Handler to verify signature
    // We construct it on the fly or get from registry. 
    // Since we need verify_webhook_signature which is specific to StripePaymentHandler (not the trait),
    // we instantiate it directly or cast. For simplicity here, we use env var directly or reuse logic.
    // Ideally, we'd get it from PaymentRegistry, but downcasting in Rust is tricky without Any.
    // We'll instantiate a temporary one since it's cheap (just holds key).
    let stripe_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_default().trim().to_string();
    let handler = StripePaymentHandler::new(stripe_key);

    match handler.verify_webhook_signature(&body, signature) {
        Ok(true) => {},
        Ok(false) => return HttpResponse::BadRequest().body("Invalid Signature"),
        Err(e) => return HttpResponse::BadRequest().body(format!("Signature verification error: {}", e)),
    };

    // 3. Parse Event
    let event: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    let event_type = event["type"].as_str().unwrap_or("");

    if event_type == "payment_intent.succeeded" {
        if let Some(data) = event["data"]["object"].as_object() {
            if let Some(metadata) = data["metadata"].as_object() {
                if let Some(sub_id_str) = metadata.get("subscription_id").and_then(|v| v.as_str()) {
                     if let Ok(sub_id) = sub_id_str.parse::<i32>() {
                         println!("WEBHOOK: Confirmed payment for subscription {}", sub_id);
                         // Call Service
                         match BillingService::confirm_subscription_payment(&pool, sub_id) {
                            Ok(sub) => {
                                // Invalidate Cache
                                let cache_key = format!("page:{}:/dashboard/settings:html", sub.tenant_id);
                                let _ = cache.delete(&cache_key).await;
                                return HttpResponse::Ok().body("Subscription Activated");
                            },
                            Err(e) => {
                                println!("WEBHOOK ERROR: {}", e);
                                return HttpResponse::InternalServerError().body("Failed to activate subscription");
                            }
                         }
                     }
                }
            }
        }
    }

    HttpResponse::Ok().body("Event Ignored")
}
