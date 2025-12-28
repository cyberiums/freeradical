use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::ai_provider_models::{AIProviderConfig, NewAIUsageLog};
use crate::models::DbPool;
use crate::schema::{ai_provider_configs, ai_usage_log};
use crate::services::errors_service::CustomHttpError;
use diesel::prelude::*;

// --- Request/Response Structs ---

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

/// Request for sentiment analysis
#[derive(Debug, Deserialize)]
pub struct AnalyzeSentimentRequest {
    pub text: String,
    pub provider_id: Option<i64>,
}

/// Response for sentiment analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct SentimentAnalysisResponse {
    pub score: f32, // -1.0 to 1.0
    pub label: String, // 'positive', 'negative', 'neutral'
    pub analysis: Option<String>,
}

/// Request for fraud detection
#[derive(Debug, Deserialize)]
pub struct FraudDetectionRequest {
    pub transaction_details: serde_json::Value,
    pub provider_id: Option<i64>,
}

/// Response for fraud detection
#[derive(Debug, Serialize, Deserialize)]
pub struct FraudDetectionResponse {
    pub risk_score: u32, // 0-100
    pub is_fraudulent: bool,
    pub reasons: Vec<String>,
}

// --- Provider Structs ---

/// OpenAI API request
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAIResponseFormat>,
}

#[derive(Debug, Serialize)]
struct OpenAIResponseFormat {
    #[serde(rename = "type")]
    response_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

// --- Helpers ---

/// Helper to get provider
async fn get_provider(pool: &web::Data<DbPool>, provider_id: Option<i64>) -> Result<AIProviderConfig, CustomHttpError> {
    web::block({
        let pool = pool.clone();
        move || -> Result<AIProviderConfig, diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string())
            ))?;
            
            if let Some(id) = provider_id {
                ai_provider_configs::table
                    .find(id as i32)
                    .first::<AIProviderConfig>(&mut conn)
            } else {
                ai_provider_configs::table
                    .filter(ai_provider_configs::is_active.eq(true))
                    .first::<AIProviderConfig>(&mut conn)
            }
        }
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Database error: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(format!("Provider not found: {}", e)))
}

/// Helper to log usage
async fn log_usage(
    req: &actix_web::HttpRequest,
    pool: &web::Data<DbPool>,
    operation: &str,
    provider: &AIProviderConfig,
    tokens: u32,
    cost_cents: u32
) -> Result<(), CustomHttpError> {
    let pool = pool.clone();
    let user_id = crate::middleware::auth_middleware::get_user_context(req)
        .map(|ctx| Some(ctx.user_id))
        .unwrap_or(None);
    let provider_type = Some(provider.provider_type.clone());
    let op = operation.to_string();
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        let log = NewAIUsageLog {
            user_id,
            operation: op,
            provider_type,
            tokens_used: Some(tokens as i32),
            cost_cents: Some(cost_cents as i32),
        };
        
        diesel::insert_into(ai_usage_log::table)
           .values(&log)
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Logging failed: {}", e)))?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))
}

// --- Provider Functions ---

/// Generate content with OpenAI
pub async fn generate_with_openai(
    provider: &AIProviderConfig,
    request: &GenerateContentRequest,
) -> Result<GeneratedContentResponse, CustomHttpError> {
    let client = Client::new();
    let api_key = provider.api_key_encrypted.clone();
    let model = provider.model_name.as_deref().unwrap_or("gpt-4");
    
    let (system_prompt, use_json) = match request.content_type.as_str() {
        "blog_post" => ("You are a professional blog writer. Generate engaging, well-structured blog posts.", false),
        "meta_description" => ("Generate SEO-optimized meta descriptions (150-160 characters).", false),
        "title" => ("Generate compelling, SEO-friendly titles (50-60 characters).", false),
        "summary" => ("Create concise, informative summaries.", false),
        "store_structure" => ("You are an expert E-commerce Store Architect. Generate a JSON structure for a complete online store based on the user's description. The JSON must have two top-level keys: 'pages' (array of objects with 'title', 'type'='page', 'content_brief') and 'categories' (array of objects with 'title', 'type'='category'). Ensure the structure is logical and SEO-friendly.", true),
        _ => ("You are a helpful AI assistant.", false),
    };
    
    let openai_req = OpenAIRequest {
        model: model.to_string(),
        messages: vec![
            OpenAIMessage { role: "system".to_string(), content: system_prompt.to_string() },
            OpenAIMessage { role: "user".to_string(), content: request.prompt.clone() },
        ],
        max_tokens: request.max_tokens.or(Some(2000)), // Higher limit for full structure
        temperature: request.temperature.or(Some(0.7)),
        response_format: if use_json { 
            Some(OpenAIResponseFormat { response_type: "json_object".to_string() })
        } else { 
            None 
        },
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&openai_req)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("request failed: {}", e)))?;
        
    let openai_res: OpenAIResponse = response.json().await
        .map_err(|e| CustomHttpError::InternalServerError(format!("parse error: {}", e)))?;
        
    let content = openai_res.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No content".to_string()))?;
        
    let cost = if model.contains("gpt-4") { 3 } else { 0 }; // Approx
    let cost_cents = (openai_res.usage.total_tokens as f32 / 1000.0 * cost as f32).ceil() as u32;

    Ok(GeneratedContentResponse {
        content,
        provider_used: "openai".to_string(),
        model: model.to_string(),
        tokens_used: openai_res.usage.total_tokens,
        cost_cents,
    })
}

/// Generate content with Anthropic
pub async fn generate_with_anthropic(
    provider: &AIProviderConfig,
    request: &GenerateContentRequest,
) -> Result<GeneratedContentResponse, CustomHttpError> {
    let client = Client::new();
    
    let model = provider.model_name.as_deref().unwrap_or("claude-3-opus-20240229");
    let endpoint = "https://api.anthropic.com/v1";
    let api_key = provider.api_key_encrypted.clone();
    
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

// --- Handlers ---

/// Analyze sentiment
pub async fn analyze_sentiment(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    payload: web::Json<AnalyzeSentimentRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_provider(&pool, payload.provider_id).await?;
    
    // Only OpenAI supported for now for structure
    if provider.provider_type != "openai" {
        return Err(CustomHttpError::BadRequest("Only OpenAI supported for sentiment analysis currently".to_string()));
    }
    
    let client = Client::new();
    let api_key = provider.api_key_encrypted.clone();
    let model = provider.model_name.as_deref().unwrap_or("gpt-4-1106-preview"); // Use preview for JSON mode
    
    let system_prompt = "Analyze the sentiment of the user input. Return a JSON object with keys: 'score' (float -1.0 to 1.0), 'label' (string: 'positive', 'negative', 'neutral'), and 'analysis' (short explanation).";
    
    let openai_req = OpenAIRequest {
        model: model.to_string(),
        messages: vec![
            OpenAIMessage { role: "system".to_string(), content: system_prompt.to_string() },
            OpenAIMessage { role: "user".to_string(), content: payload.text.clone() },
        ],
        max_tokens: Some(500),
        temperature: Some(0.0),
        response_format: Some(OpenAIResponseFormat { response_type: "json_object".to_string() }),
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&openai_req)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Request failed: {}", e)))?;
        
    let openai_res: OpenAIResponse = response.json().await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Parse error: {}", e)))?;
        
    let content_str = openai_res.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No content".to_string()))?;
        
    let result: SentimentAnalysisResponse = serde_json::from_str(&content_str)
        .map_err(|e| CustomHttpError::InternalServerError(format!("JSON parse error: {}", e)))?;
        
    // Log usage
    log_usage(&req, &pool, "analyze_sentiment", &provider, openai_res.usage.total_tokens, 1).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// Detect fraud
pub async fn detect_fraud(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    payload: web::Json<FraudDetectionRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_provider(&pool, payload.provider_id).await?;
    
    if provider.provider_type != "openai" {
        return Err(CustomHttpError::BadRequest("Only OpenAI supported for fraud detection currently".to_string()));
    }
    
    let client = Client::new();
    let api_key = provider.api_key_encrypted.clone();
    let model = provider.model_name.as_deref().unwrap_or("gpt-4-1106-preview");
    
    let system_prompt = "You are a fraud detection expert. Analyze the transaction details provided in JSON. Return a JSON object with: 'risk_score' (0-100 integer), 'is_fraudulent' (boolean), and 'reasons' (array of strings explaining the score).";
    
    let transaction_str = serde_json::to_string(&payload.transaction_details).unwrap_or_default();
    
    let openai_req = OpenAIRequest {
        model: model.to_string(),
        messages: vec![
            OpenAIMessage { role: "system".to_string(), content: system_prompt.to_string() },
            OpenAIMessage { role: "user".to_string(), content: transaction_str },
        ],
        max_tokens: Some(500),
        temperature: Some(0.0),
        response_format: Some(OpenAIResponseFormat { response_type: "json_object".to_string() }),
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&openai_req)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Request failed: {}", e)))?;
        
    let openai_res: OpenAIResponse = response.json().await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Parse error: {}", e)))?;
        
    let content_str = openai_res.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No content".to_string()))?;
        
    let result: FraudDetectionResponse = serde_json::from_str(&content_str)
        .map_err(|e| CustomHttpError::InternalServerError(format!("JSON parse error: {}", e)))?;
        
    log_usage(&req, &pool, "detect_fraud", &provider, openai_res.usage.total_tokens, 2).await?;
    
    Ok(HttpResponse::Ok().json(result))
}


// Re-implement generate_content to match signature and keep existing functionality
pub async fn generate_content(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateContentRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_provider(&pool, payload.provider_id).await?;
    
    let result = match provider.provider_type.as_str() {
        "openai" => generate_with_openai(&provider, &payload).await?,
        "anthropic" => generate_with_anthropic(&provider, &payload).await?,
        "gcp" => return Err(CustomHttpError::BadRequest("GCP not yet implemented".to_string())),
        "azure" => return Err(CustomHttpError::BadRequest("Azure not yet implemented".to_string())),
        _ => return Err(CustomHttpError::BadRequest("Unsupported provider".to_string())),
    };

    log_usage(&req, &pool, "generate_content", &provider, result.tokens_used, result.cost_cents).await?;
    
    Ok(HttpResponse::Ok().json(result))
}
