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

pub async fn list_payment_handlers(
    registry: web::Data<PaymentHandlerRegistry>,
) -> Result<HttpResponse, CustomHttpError> {
    let handlers = registry.list_handlers();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "providers": handlers
    })))
}
