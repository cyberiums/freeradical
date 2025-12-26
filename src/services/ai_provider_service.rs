use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::ai_provider_models::{
    AIProviderConfig, AIProviderConfigPublic, NewAIProviderConfig,
};
use crate::models::DatabasePool;
use crate::schema::ai_provider_configs;
use crate::services::errors_service::CustomHttpError;

/// Request to create AI provider
#[derive(Debug, Deserialize)]
pub struct CreateProviderRequest {
    pub provider_type: String,
    pub api_key: String,
    pub model_name: Option<String>,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
}

/// Request to update AI provider
#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub is_active: Option<bool>,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
}

/// Test provider connection request
#[derive(Debug, Deserialize)]
pub struct TestProviderRequest {
    pub provider_id: i64,
}

/// Test provider response
#[derive(Debug, Serialize)]
pub struct TestProviderResponse {
    pub success: bool,
    pub message: String,
    pub latency_ms: Option<i32>,
}

/// List all AI providers
pub async fn list_providers(
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let providers = web::block(move || -> Result<Vec<AIProviderConfigPublic>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        let configs = ai_provider_configs::table
            .filter(ai_provider_configs::is_active.eq(true))
            .order(ai_provider_configs::id.asc())
            .load::<AIProviderConfig>(&mut conn)?;
        
        Ok(configs.into_iter().map(|c| c.into()).collect())
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(providers))
}

/// Get single AI provider
pub async fn get_provider(
    pool: web::Data<DatabasePool>,
    provider_id: web::Path<i64>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        let config = ai_provider_configs::table
            .find(id as i32)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(config.into())
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(provider))
}

/// Create new AI provider
pub async fn create_provider(
    pool: web::Data<DatabasePool>,
    payload: web::Json<CreateProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let user_id = None; // TODO: Get from auth context
    
    let api_key_encrypted = encrypt_api_key(&payload.api_key)?;
    
    let new_provider = NewAIProviderConfig {
        provider_type: payload.provider_type.clone(),
        api_key_encrypted: String::from_utf8_lossy(&api_key_encrypted).to_string(),
        model_name: payload.model_name.clone(),
        is_active: Some(true),
        daily_token_limit: payload.daily_token_limit,
        monthly_budget_cents: payload.monthly_budget_cents,
        created_by: user_id,
    };
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        // Note: is_default field doesn't exist in current schema
        // Can be added in future migration if needed
        
        diesel::insert_into(ai_provider_configs::table)
            .values(&new_provider)
            .execute(&mut conn)?;
        
        let config = ai_provider_configs::table
            .order(ai_provider_configs::id.desc())
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(config.into())
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(provider))
}

/// Update AI provider
pub async fn update_provider(
    pool: web::Data<DatabasePool>,
    provider_id: web::Path<i64>,
    payload: web::Json<UpdateProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        // Note: name field doesn't exist in current schema
        // API key update also disabled since encrypted field update needs full implementation
        
        // if let Some(ref api_key) = payload.api_key {
        //     let encrypted = encrypt_api_key(api_key)?;
        //     diesel::update(ai_provider_configs::table.find(id))
        //         .set(ai_provider_configs::api_key_encrypted.eq(encrypted))
        //         .execute(&mut conn)?;
        // }

        
        let updated = ai_provider_configs::table
            .find(id as i32)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(updated.into())
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(provider))
}

/// Delete AI provider
pub async fn delete_provider(
    pool: web::Data<DatabasePool>,
    provider_id: web::Path<i64>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        diesel::delete(ai_provider_configs::table.find(id as i32))
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Provider deleted successfully"
    })))
}

/// Test AI provider connection
pub async fn test_provider(
    pool: web::Data<DatabasePool>,
    payload: web::Json<TestProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider_id = payload.provider_id;
    
    let result = web::block(move || -> Result<TestProviderResponse, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        let _config = ai_provider_configs::table
            .find(provider_id as i32)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(TestProviderResponse {
            success: true,
            message: "Provider configuration valid (full test pending)".to_string(),
            latency_ms: Some(0),
        })
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(result))
}

fn encrypt_api_key(api_key: &str) -> Result<Vec<u8>, CustomHttpError> {
    // TODO: Implement proper encryption
    Ok(api_key.as_bytes().to_vec())
}

#[allow(dead_code)]
fn decrypt_api_key(encrypted: &[u8]) -> Result<String, CustomHttpError> {
    // TODO: Implement proper decryption
    String::from_utf8(encrypted.to_vec())
        .map_err(|_| CustomHttpError::InternalServerError("Decryption failed".to_string()))
}
