use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::ai_provider_models::{
    AIProviderConfig, AIProviderConfigPublic, NewAIProviderConfig,
};
use crate::models::DbPool;
use crate::schema::ai_provider_configs;
use crate::services::errors_service::CustomHttpError;

/// Request to create AI provider
#[derive(Debug, Deserialize)]
pub struct CreateProviderRequest {
    pub provider_type: String,
    pub name: String,
    pub api_key: String,
    pub config: serde_json::Value,
    pub is_default: Option<bool>,
    pub priority: Option<i32>,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
}

/// Request to update AI provider
#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub priority: Option<i32>,
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
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let providers = web::block(move || -> Result<Vec<AIProviderConfigPublic>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        let configs = ai_provider_configs::table
            .filter(ai_provider_configs::is_active.eq(true))
            .order(ai_provider_configs::priority.asc())
            .load::<AIProviderConfig>(&mut conn)?;
        
        Ok(configs.into_iter().map(|c| c.into()).collect())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(providers))
}

/// Get single AI provider
pub async fn get_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i64>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        let config = ai_provider_configs::table
            .find(id)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(config.into())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(provider))
}

/// Create new AI provider
pub async fn create_provider(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Get user_id from auth context
    let user_id = None;
    
    // Encrypt API key
    let api_key_encrypted = encrypt_api_key(&payload.api_key)?;
    
    let new_provider = NewAIProviderConfig {
        provider_type: payload.provider_type.clone(),
        name: payload.name.clone(),
        api_key_encrypted: Some(api_key_encrypted),
        config: payload.config.clone(),
        is_active: Some(true),
        is_default: payload.is_default,
        priority: payload.priority,
        daily_token_limit: payload.daily_token_limit,
        monthly_budget_cents: payload.monthly_budget_cents,
        created_by: user_id,
    };
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        // If setting as default, unset other defaults
        if new_provider.is_default == Some(true) {
            diesel::update(ai_provider_configs::table)
                .set(ai_provider_configs::is_default.eq(false))
                .execute(&mut conn)?;
        }
        
        diesel::insert_into(ai_provider_configs::table)
            .values(&new_provider)
            .execute(&mut conn)?;
        
        let config = ai_provider_configs::table
            .order(ai_provider_configs::id.desc())
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(config.into())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(provider))
}

/// Update AI provider
pub async fn update_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i64>,
    payload: web::Json<UpdateProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let provider = web::block(move || -> Result<AIProviderConfigPublic, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        // If setting as default, unset other defaults
        if payload.is_default == Some(true) {
            diesel::update(ai_provider_configs::table)
                .filter(ai_provider_configs::id.ne(id))
                .set(ai_provider_configs::is_default.eq(false))
                .execute(&mut conn)?;
        }
        
        // Build update
        if let Some(ref name) = payload.name {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::name.eq(name))
                .execute(&mut conn)?;
        }
        
        if let Some(ref api_key) = payload.api_key {
            let encrypted = encrypt_api_key(api_key).map_err(|_| diesel::result::Error::RollbackTransaction)?;
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::api_key_encrypted.eq(encrypted))
                .execute(&mut conn)?;
        }
        
        if let Some(ref config) = payload.config {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::config.eq(config))
                .execute(&mut conn)?;
        }
        
        if let Some(is_active) = payload.is_active {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::is_active.eq(is_active))
                .execute(&mut conn)?;
        }
        
        if let Some(is_default) = payload.is_default {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::is_default.eq(is_default))
                .execute(&mut conn)?;
        }
        
        if let Some(priority) = payload.priority {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::priority.eq(priority))
                .execute(&mut conn)?;
        }
        
        let updated = ai_provider_configs::table
            .find(id)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(updated.into())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(provider))
}

/// Delete AI provider
pub async fn delete_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i64>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        diesel::delete(ai_provider_configs::table.find(id))
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Provider deleted successfully"
    })))
}

/// Test AI provider connection
pub async fn test_provider(
    pool: web::Data<DbPool>,
    payload: web::Json<TestProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider_id = payload.provider_id;
    
    // TODO: Implement actual provider testing
    // For now, just check if provider exists and has API key
    let result = web::block(move || -> Result<TestProviderResponse, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        let _config = ai_provider_configs::table
            .find(provider_id)
            .first::<AIProviderConfig>(&mut conn)?;
        
        Ok(TestProviderResponse {
            success: true,
            message: "Provider configuration valid (full test pending)".to_string(),
            latency_ms: Some(0),
        })
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(result))
}

/// Encrypt API key for storage
fn encrypt_api_key(api_key: &str) -> Result<Vec<u8>, CustomHttpError> {
    // TODO: Implement proper encryption using app secret key
    // For now, just convert to bytes (NOT SECURE - placeholder only)
    Ok(api_key.as_bytes().to_vec())
}

/// Decrypt API key from storage
#[allow(dead_code)]
fn decrypt_api_key(encrypted: &[u8]) -> Result<String, CustomHttpError> {
    // TODO: Implement proper decryption
    // For now, just convert from bytes (NOT SECURE - placeholder only)
    String::from_utf8(encrypted.to_vec())
        .map_err(|_| CustomHttpError::InternalServerError("Decryption failed".to_string()))
}
