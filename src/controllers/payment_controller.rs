use actix_web::{web, HttpResponse};
use crate::services::payment_service::{PaymentHandlerRegistry, CreatePaymentIntentRequest};
use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;
use crate::models::DatabasePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub amount_cents: i64,
    pub currency: String,
    pub provider: String, // "stripe", "paypal", etc.
    pub metadata: Option<HashMap<String, String>>,
}

/// Create payment intent with provider
#[utoipa::path(
    post,
    path = "/v1/payments/intent",
    tag = "Commerce - Payments",
    request_body = CreatePaymentRequest,
    responses(
        (status = 200, description = "Payment intent created (returns provider-specific intent ID)"),
        (status = 400, description = "Invalid provider or payment data"),
        (status = 500, description = "Payment provider error"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_payment_intent(
    body: web::Json<CreatePaymentRequest>,
    registry: web::Data<PaymentHandlerRegistry>,
    _pool: web::Data<DatabasePool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let handler = registry
        .get(&body.provider)
        .ok_or(CustomHttpError::BadRequest("Payment handler not found".to_string()))?;
    
    let request = CreatePaymentIntentRequest {
        amount_cents: body.amount_cents,
        currency: body.currency.clone(),
        metadata: body.metadata.clone().unwrap_or_default(),
    };
    
    let intent = handler
        .create_payment_intent(request)
        .await
        .map_err(|e| {
            log::error!("Payment creation failed: {}", e);
            CustomHttpError::InternalServerError("Payment creation failed".to_string())
        })?;
    
    Ok(HttpResponse::Ok().json(intent))
}

#[derive(Deserialize)]
pub struct GetPaymentRequest {
    pub provider: String,
    pub intent_id: String,
}

/// Get payment intent status
#[utoipa::path(
    get,
    path = "/v1/payments/intent",
    tag = "Commerce - Payments",
    params(
        ("provider" = String, Query, description = "Payment provider (stripe, paypal, square)", example = "stripe"),
        ("intent_id" = String, Query, description = "Provider payment intent ID", example = "pi_1234567890")
    ),
    responses(
        (status = 200, description = "Payment intent details"),
        (status = 400, description = "Invalid provider"),
        (status = 500, description = "Provider error"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_payment_intent(
    query: web::Query<GetPaymentRequest>,
    registry: web::Data<PaymentHandlerRegistry>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let handler = registry
        .get(&query.provider)
        .ok_or(CustomHttpError::BadRequest("Payment handler not found".to_string()))?;
    
    let intent = handler
        .get_payment_intent(&query.intent_id)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch payment intent: {}", e);
            CustomHttpError::InternalServerError("Failed to fetch payment intent".to_string())
        })?;
    
    Ok(HttpResponse::Ok().json(intent))
}

/// List available payment providers
#[utoipa::path(
    get,
    path = "/v1/payments/providers",
    tag = "Commerce - Payments",
    responses(
        (status = 200, description = "List of configured payment providers (stripe, paypal, square)")
    )
)]
pub async fn list_payment_handlers(
    registry: web::Data<PaymentHandlerRegistry>,
) -> Result<HttpResponse, CustomHttpError> {
    let handlers = registry.list_handlers();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "providers": handlers
    })))
}
