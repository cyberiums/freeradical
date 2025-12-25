use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::DbPool;
use crate::services::ai_content_service::{generate_with_openai, generate_with_anthropic};
use crate::services::errors_service::CustomHttpError;
use crate::models::ai_provider_models::AIProviderConfig;
use crate::schema::ai_provider_configs;

/// Request to generate metadata
#[derive(Debug, Deserialize)]
pub struct GenerateMetadataRequest {
    pub content: String,
    pub metadata_type: String, // 'keywords', 'tags', 'categories', 'alt_text'
    pub image_url: Option<String>, // For alt text generation
    pub max_items: Option<usize>,
}

/// Metadata response
#[derive(Debug, Serialize)]
pub struct MetadataResponse {
    pub keywords: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub alt_text: Option<String>,
    pub confidence: f32,
}

/// Extract SEO keywords from content
pub async fn extract_keywords(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateMetadataRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_default_provider(pool.clone()).await?;
    
    let max_keywords = payload.max_items.unwrap_or(10);
    let prompt = format!(
        "Extract {} SEO keywords from this content. Return ONLY a comma-separated list of keywords:\n\n{}",
        max_keywords,
        &payload.content
    );
    
    let content = generate_ai_content(&provider, &prompt).await?;
    let keywords: Vec<String> = content
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .take(max_keywords)
        .collect();
    
    Ok(HttpResponse::Ok().json(MetadataResponse {
        keywords: Some(keywords),
        tags: None,
        categories: None,
        alt_text: None,
        confidence: 0.85,
    }))
}

/// Generate tags from content
pub async fn generate_tags(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateMetadataRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_default_provider(pool.clone()).await?;
    
    let max_tags = payload.max_items.unwrap_or(8);
    let prompt = format!(
        "Generate {} content tags for this text. Return ONLY a comma-separated list:\n\n{}",
        max_tags,
        &payload.content
    );
    
    let content = generate_ai_content(&provider, &prompt).await?;
    let tags: Vec<String> = content
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .take(max_tags)
        .collect();
    
    Ok(HttpResponse::Ok().json(MetadataResponse {
        keywords: None,
        tags: Some(tags),
        categories: None,
        alt_text: None,
        confidence: 0.8,
    }))
}

/// Suggest categories from content
pub async fn suggest_categories(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateMetadataRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_default_provider(pool.clone()).await?;
    
    let max_cats = payload.max_items.unwrap_or(5);
    let prompt = format!(
        "Suggest {} content categories for this text. Return ONLY a comma-separated list:\n\n{}",
        max_cats,
        &payload.content
    );
    
    let content = generate_ai_content(&provider, &prompt).await?;
    let categories: Vec<String> = content
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .take(max_cats)
        .collect();
    
    Ok(HttpResponse::Ok().json(MetadataResponse {
        keywords: None,
        tags: None,
        categories: Some(categories),
        alt_text: None,
        confidence: 0.75,
    }))
}

/// Generate image alt text
pub async fn generate_alt_text(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateMetadataRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_default_provider(pool.clone()).await?;
    
    let image_context = payload.image_url.clone().unwrap_or_else(|| "image".to_string());
    let prompt = format!(
        "Generate a concise, SEO-friendly alt text description (max 125 characters) for an image in this context:\n\nContext: {}\nImage: {}",
        &payload.content,
        image_context
    );
    
    let alt_text = generate_ai_content(&provider, &prompt).await?;
    let alt_text = alt_text.trim().trim_matches('"').to_string();
    
    Ok(HttpResponse::Ok().json(MetadataResponse {
        keywords: None,
        tags: None,
        categories: None,
        alt_text: Some(alt_text),
        confidence: 0.9,
    }))
}

/// Generate all metadata at once
pub async fn generate_all_metadata(
    pool: web::Data<DbPool>,
    payload: web::Json<GenerateMetadataRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let provider = get_default_provider(pool.clone()).await?;
    
    let prompt = format!(
        r#"Analyze this content and provide:
1. 5 SEO keywords (comma-separated)
2. 5 content tags (comma-separated)
3. 3 category suggestions (comma-separated)

Content:
{}

Format your response as:
KEYWORDS: keyword1, keyword2, keyword3
TAGS: tag1, tag2, tag3
CATEGORIES: cat1, cat2, cat3
"#,
        &payload.content
    );
    
    let response = generate_ai_content(&provider, &prompt).await?;
    
    // Parse response
    let mut keywords = Vec::new();
    let mut tags = Vec::new();
    let mut categories = Vec::new();
    
    for line in response.lines() {
        let line = line.trim();
        if line.starts_with("KEYWORDS:") {
            keywords = line.replace("KEYWORDS:", "")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        } else if line.starts_with("TAGS:") {
            tags = line.replace("TAGS:", "")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        } else if line.starts_with("CATEGORIES:") {
            categories = line.replace("CATEGORIES:", "")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    
    Ok(HttpResponse::Ok().json(MetadataResponse {
        keywords: Some(keywords),
        tags: Some(tags),
        categories: Some(categories),
        alt_text: None,
        confidence: 0.8,
    }))
}

/// Get default AI provider
async fn get_default_provider(pool: web::Data<DbPool>) -> Result<AIProviderConfig, CustomHttpError> {
    web::block(move || -> Result<AIProviderConfig, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Database connection error".to_string())
        ))?;
        
        ai_provider_configs::table
            .filter(ai_provider_configs::is_default.eq(true))
            .filter(ai_provider_configs::is_active.eq(true))
            .first::<AIProviderConfig>(&mut conn)
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(format!("No active provider: {}", e)))
}

/// Generate AI content using provider
async fn generate_ai_content(provider: &AIProviderConfig, prompt: &str) -> Result<String, CustomHttpError> {
    use crate::services::ai_content_service::GenerateContentRequest;
    
    let request = GenerateContentRequest {
        prompt: prompt.to_string(),
        content_type: "metadata".to_string(),
        max_tokens: Some(200),
        temperature: Some(0.3), // Lower temperature for more consistent results
        provider_id: None,
    };
    
    let result = match provider.provider_type.as_str() {
        "openai" => generate_with_openai(provider, &request).await?,
        "anthropic" => generate_with_anthropic(provider, &request).await?,
        _ => return Err(CustomHttpError::BadRequest("Unsupported provider".to_string())),
    };
    
    Ok(result.content)
}
