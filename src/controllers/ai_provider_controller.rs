use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::ai_provider_models::{AIProviderConfig, NewAIProviderConfig};
use crate::models::DbPool;
use crate::schema::{ai_provider_configs, ai_usage_log};
use crate::services::errors_service::CustomHttpError;

/// AI Provider response
#[derive(Debug, Serialize, ToSchema)]
pub struct AIProviderResponse {
    pub id: i32,
    pub provider_type: String,
    pub model_name: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: String,
}

impl From<AIProviderConfig> for AIProviderResponse {
    fn from(config: AIProviderConfig) -> Self {
        AIProviderResponse {
            id: config.id,
            provider_type: config.provider_type,
            model_name: config.model_name,
            is_active: config.is_active,
            created_at: config.created_at.to_string(),
        }
    }
}

/// Create AI Provider request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAIProviderRequest {
    pub provider_type: String,
    pub api_key: String,
    pub model_name: Option<String>,
    pub is_active: Option<bool>,
}

/// Update AI Provider request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAIProviderRequest {
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub is_active: Option<bool>,
}

/// AI Provider usage stats
#[derive(Debug, Serialize, ToSchema)]
pub struct AIProviderUsageStats {
    pub provider_id: i32,
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost_cents: i64,
    pub last_used_at: Option<String>,
}

/// List all AI providers
#[utoipa::path(
    get,
    path = "/v1/admin/ai-providers",
    tag = "Internal - AI Providers",
    responses(
        (status = 200, description = "List of AI providers", body = Vec<AIProviderResponse>),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_ai_providers(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let providers = web::block(move || -> Result<Vec<AIProviderConfig>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        ai_provider_configs::table
            .order(ai_provider_configs::created_at.desc())
            .load::<AIProviderConfig>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?;
    
    let response: Vec<AIProviderResponse> = providers.into_iter().map(AIProviderResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// Create new AI provider
#[utoipa::path(
    post,
    path = "/v1/admin/ai-providers",
    tag = "Internal - AI Providers",
    request_body = CreateAIProviderRequest,
    responses(
        (status = 201, description = "AI provider created", body = AIProviderResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_ai_provider(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateAIProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_provider = NewAIProviderConfig {
        provider_type: payload.provider_type.clone(),
        api_key_encrypted: payload.api_key.clone(),
        model_name: payload.model_name.clone(),
        is_active: Some(payload.is_active.unwrap_or(true)),
        daily_token_limit: None,
        monthly_budget_cents: None,
        created_by: None,
    };
    
    let config = web::block(move || -> Result<AIProviderConfig, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        diesel::insert_into(ai_provider_configs::table)
            .values(&new_provider)
            .get_result::<AIProviderConfig>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?;
    
    Ok(HttpResponse::Created().json(AIProviderResponse::from(config)))
}

/// Update AI provider
#[utoipa::path(
    put,
    path = "/v1/admin/ai-providers/{id}",
    tag = "Internal - AI Providers",
    params(
        ("id" = i32, Path, description = "AI Provider ID")
    ),
    request_body = UpdateAIProviderRequest,
    responses(
        (status = 200, description = "AI provider updated", body = AIProviderResponse),
        (status = 404, description = "Provider not found"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_ai_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i32>,
    payload: web::Json<UpdateAIProviderRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let config = web::block(move || -> Result<AIProviderConfig, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        if let Some(ref api_key) = payload.api_key {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::api_key_encrypted.eq(api_key))
                .execute(&mut conn)?;
        }
        if let Some(ref model) = payload.model_name {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::model_name.eq(model))
                .execute(&mut conn)?;
        }
        if let Some(active) = payload.is_active {
            diesel::update(ai_provider_configs::table.find(id))
                .set(ai_provider_configs::is_active.eq(Some(active)))
                .execute(&mut conn)?;
        }
        
        ai_provider_configs::table.find(id)
            .first::<AIProviderConfig>(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => CustomHttpError::NotFound("Provider not found".to_string()),
        _ => CustomHttpError::InternalServerError(format!("Database error: {}", e)),
    })?;
    
    Ok(HttpResponse::Ok().json(AIProviderResponse::from(config)))
}

/// Delete AI provider
#[utoipa::path(
    delete,
    path = "/v1/admin/ai-providers/{id}",
    tag = "Internal - AI Providers",
    params(
        ("id" = i32, Path, description = "AI Provider ID")
    ),
    responses(
        (status = 204, description = "AI provider deleted"),
        (status = 404, description = "Provider not found"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_ai_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let deleted = web::block(move || -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        diesel::delete(ai_provider_configs::table.find(id))
            .execute(&mut conn)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?;
    
    if deleted == 0 {
        return Err(CustomHttpError::NotFound("Provider not found".to_string()));
    }
    
    Ok(HttpResponse::NoContent().finish())
}

/// Get AI provider usage statistics
#[utoipa::path(
    get,
    path = "/v1/admin/ai-providers/{id}/usage",
    tag = "Internal - AI Providers",
    params(
        ("id" = i32, Path, description = "AI Provider ID")
    ),
    responses(
        (status = 200, description = "Usage statistics", body = AIProviderUsageStats),
        (status = 404, description = "Provider not found"),
        (status = 500, description = "Database error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_provider_usage(
    pool: web::Data<DbPool>,
    provider_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    let id = provider_id.into_inner();
    
    let stats = web::block(move || -> Result<AIProviderUsageStats, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        // Verify provider exists
        ai_provider_configs::table.find(id)
            .first::<AIProviderConfig>(&mut conn)?;
        
        // Get usage stats (simplified - in production you'd join with usage log)
        let total_requests = ai_usage_log::table
            .filter(ai_usage_log::user_id.is_not_null())
            .count()
            .get_result::<i64>(&mut conn)?;
            
        Ok(AIProviderUsageStats {
            provider_id: id,
            total_requests,
            total_tokens: 0,
            total_cost_cents: 0,
            last_used_at: None,
        })
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Blocking error: {}", e)))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => CustomHttpError::NotFound("Provider not found".to_string()),
        _ => CustomHttpError::InternalServerError(format!("Database error: {}", e)),
    })?;
    
    Ok(HttpResponse::Ok().json(stats))
}
