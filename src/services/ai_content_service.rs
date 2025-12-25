use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::ai_provider_models::{AIProviderConfig, AIUsageLog, NewAIUsageLog};
use crate::models::DbPool;
use crate::schema::{ai_provider_configs, ai_usage_log};
use crate::services::errors_service::CustomHttpError;

/// Request to generate content
#[derive(Debug, Deserialize)]
pub struct GenerateContentRequest {
    pub prompt: String,
    pub content_type: String, // 'blog_post', 'meta_description', 'title', 'summary'
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub provider_id: Option<i64>, // If None, use default provider
}

/// Generated content response
#[derive(Debug, Serialize)]
pub struct GeneratedContentResponse {
    pub content: String,
    pub provider_used: String,
    pub model: String,
    pub tokens_used: u32,
    pub cost_cents: u32,
}

/// OpenAI API request
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

/// OpenAI API response
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Anthropic API request
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

/// Anthropic API response
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

/// Generate content using AI
pub async fn generate_content(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateContentRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    // Get provider
    let provider_id = payload.provider_id;
    
    let provider = web::block(move || -> Result<AIProviderConfig, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        if let Some(id) = provider_id {
            ai_provider_configs::table
                .find(id)
                .first::<AIProviderConfig>(&mut conn)
        } else {
            ai_provider_configs::table
                .filter(ai_provider_configs::is_default.eq(true))
                .first::<AIProviderConfig>(&mut conn)
        }
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Provider not found: {}", e)))?;
    
    // Generate content based on provider type
    let result = match provider.provider_type.as_str() {
        "openai" => generate_with_openai(&provider, &payload).await?,
        "anthropic" => generate_with_anthropic(&provider, &payload).await?,
        "gcp" => return Err(CustomHttpError::BadRequest("GCP not yet implemented".to_string())),
        "azure" => generate_with_azure(&provider, &payload).await?,
        _ => return Err(CustomHttpError::BadRequest("Unsupported provider".to_string())),
    };
    
    // Log usage
    let pool_clone = pool.clone();
    let provider_id = provider.id;
    let log = NewAIUsageLog {
        provider_id: Some(provider_id),
        user_id: None, // TODO: Get from auth context
        operation: "generate_content".to_string(),
        prompt_tokens: result.tokens_used as i32,
        completion_tokens: 0,
        total_tokens: result.tokens_used as i32,
        cost_cents: result.cost_cents as i32,
        model: result.model.clone(),
        latency_ms: 0,
        success: true,
        error: None,
    };
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool_clone.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        diesel::insert_into(ai_usage_log::table)
            .values(&log)
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// Generate content with OpenAI
async fn generate_with_openai(
    provider: &AIProviderConfig,
    request: &GenerateContentRequest,
) -> Result<GeneratedContentResponse, CustomHttpError> {
    let client = Client::new();
    
    // Get config
    let model = provider.config.get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("gpt-4");
    let endpoint = provider.config.get("endpoint")
        .and_then(|v| v.as_str())
        .unwrap_or("https://api.openai.com/v1");
    
    // Decrypt API key
    let api_key = decrypt_api_key(&provider.api_key_encrypted.as_ref().unwrap())?;
    
    // Build system prompt based on content type
    let system_prompt = match request.content_type.as_str() {
        "blog_post" => "You are a professional blog writer. Generate engaging, well-structured blog posts.",
        "meta_description" => "Generate SEO-optimized meta descriptions (150-160 characters).",
        "title" => "Generate compelling, SEO-friendly titles (50-60 characters).",
        "summary" => "Create concise, informative summaries.",
        _ => "You are a helpful AI assistant.",
    };
    
    let openai_request = OpenAIRequest {
        model: model.to_string(),
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: request.prompt.clone(),
            },
        ],
        max_tokens: request.max_tokens.or(Some(1000)),
        temperature: request.temperature.or(Some(0.7)),
    };
    
    let response = client
        .post(format!("{}/chat/completions", endpoint))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&openai_request)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("OpenAI request failed: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(CustomHttpError::InternalServerError(format!("OpenAI error: {}", error_text)));
    }
    
    let openai_response: OpenAIResponse = response
        .json()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to parse OpenAI response: {}", e)))?;
    
    let content = openai_response.choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No content in response".to_string()))?;
    
    // Estimate cost (GPT-4: ~$0.03/1K tokens, GPT-3.5: ~$0.002/1K tokens)
    let cost_per_1k_tokens = if model.contains("gpt-4") { 3 } else { 0 };
    let cost_cents = (openai_response.usage.total_tokens as f32 / 1000.0 * cost_per_1k_tokens as f32).ceil() as u32;
    
    Ok(GeneratedContentResponse {
        content,
        provider_used: "openai".to_string(),
        model: model.to_string(),
        tokens_used: openai_response.usage.total_tokens,
        cost_cents,
    })
}

/// Generate content with Anthropic
async fn generate_with_anthropic(
    provider: &AIProviderConfig,
    request: &GenerateContentRequest,
) -> Result<GeneratedContentResponse, CustomHttpError> {
    let client = Client::new();
    
    let model = provider.config.get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("claude-3-opus-20240229");
    let endpoint = provider.config.get("endpoint")
        .and_then(|v| v.as_str())
        .unwrap_or("https://api.anthropic.com/v1");
    
    let api_key = decrypt_api_key(&provider.api_key_encrypted.as_ref().unwrap())?;
    
    let anthropic_request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: request.max_tokens.unwrap_or(1000),
        messages: vec![
            AnthropicMessage {
                role: "user".to_string(),
                content: request.prompt.clone(),
            },
        ],
        temperature: request.temperature,
    };
    
    let response = client
        .post(format!("{}/messages", endpoint))
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&anthropic_request)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Anthropic request failed: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(CustomHttpError::InternalServerError(format!("Anthropic error: {}", error_text)));
    }
    
    let anthropic_response: AnthropicResponse = response
        .json()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to parse Anthropic response: {}", e)))?;
    
    let content = anthropic_response.content
        .first()
        .map(|c| c.text.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No content in response".to_string()))?;
    
    let total_tokens = anthropic_response.usage.input_tokens + anthropic_response.usage.output_tokens;
    let cost_cents = (total_tokens as f32 / 1000.0 * 1.5).ceil() as u32; // Estimate
    
    Ok(GeneratedContentResponse {
        content,
        provider_used: "anthropic".to_string(),
        model: model.to_string(),
        tokens_used: total_tokens,
        cost_cents,
    })
}

/// Generate content with Azure OpenAI
async fn generate_with_azure(
    provider: &AIProviderConfig,
    request: &GenerateContentRequest,
) -> Result<GeneratedContentResponse, CustomHttpError> {
    // Azure uses same API format as OpenAI but different endpoint structure
    generate_with_openai(provider, request).await
}

/// Decrypt API key (placeholder - implement proper encryption)
fn decrypt_api_key(encrypted: &[u8]) -> Result<String, CustomHttpError> {
    String::from_utf8(encrypted.to_vec())
        .map_err(|_| CustomHttpError::InternalServerError("Decryption failed".to_string()))
}
