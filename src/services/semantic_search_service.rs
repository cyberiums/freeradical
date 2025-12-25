use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use sha2::{Sha256, Digest};

use crate::models::DbPool;
use crate::models::ai_provider_models::AIProviderConfig;
use crate::schema::{ai_provider_configs, content_embeddings, search_history};
use crate::services::errors_service::CustomHttpError;

/// Vector embedding (1536 dimensions for OpenAI ada-002)
pub type Embedding = Vec<f32>;

/// Request to create embedding
#[derive(Debug, Deserialize)]
pub struct CreateEmbeddingRequest {
    pub page_id: i64,
    pub content: String,
}

/// Search request
#[derive(Debug, Deserialize)]
pub struct SemanticSearchRequest {
    pub query: String,
    pub limit: Option<i64>,
    pub min_similarity: Option<f32>,
}

/// Search result
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub page_id: i64,
    pub content_preview: String,
    pub similarity: f32,
    pub rank: i32,
}

/// Search response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub search_type: String,
}

/// Content embedding model
#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = content_embeddings)]
pub struct ContentEmbedding {
    pub id: Option<i64>,
    pub page_id: i64,
    pub content_hash: String,
    pub embedding: Vec<f32>,
    pub content_preview: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Create or update embedding for content
pub async fn create_embedding(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateEmbeddingRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let content = payload.content.clone();
    let page_id = payload.page_id;
    
    // Generate content hash
    let content_hash = generate_content_hash(&content);
    
    // Check if embedding already exists
    let existing = web::block({
        let pool = pool.clone();
        let hash = content_hash.clone();
        move || -> Result<Option<i64>, diesel::result::Error> {
            let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Connection error".to_string())
            ))?;
            
            content_embeddings::table
                .filter(content_embeddings::page_id.eq(page_id))
                .filter(content_embeddings::content_hash.eq(hash))
                .select(content_embeddings::id)
                .first::<i64>(&mut conn)
                .optional()
        }
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    if existing.is_some() {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Embedding already exists",
            "id": existing.unwrap()
        })));
    }
    
    // Generate embedding using AI provider
    let embedding = generate_embedding_vector(&content).await?;
    
    // Get content preview (first 200 chars)
    let preview = content.chars().take(200).collect::<String>();
    
    // Store embedding
    let id = web::block(move || -> Result<i64, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        diesel::insert_into(content_embeddings::table)
            .values((
                content_embeddings::page_id.eq(page_id),
                content_embeddings::content_hash.eq(content_hash),
                content_embeddings::embedding.eq(embedding),
                content_embeddings::content_preview.eq(preview),
            ))
            .returning(content_embeddings::id)
            .get_result(&mut conn)
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Embedding created",
        "id": id
    })))
}

/// Perform semantic search
pub async fn semantic_search(
    pool: web::Data<DbPool>,
    payload: web::Json<SemanticSearchRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let query = payload.query.clone();
    let limit = payload.limit.unwrap_or(10);
    let min_similarity = payload.min_similarity.unwrap_or(0.7);
    
    // Generate query embedding
    let query_embedding = generate_embedding_vector(&query).await?;
    
    // Perform vector similarity search
    let results = web::block(move || -> Result<Vec<SearchResult>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        // Use cosine similarity search
        // SQL: SELECT *, 1 - (embedding <=> query_vector) AS similarity
        let sql = format!(
            "SELECT page_id, content_preview, 1 - (embedding <=> '[{}]') AS similarity 
             FROM content_embeddings 
             WHERE 1 - (embedding <=> '[{}]') > {} 
             ORDER BY embedding <=> '[{}]' 
             LIMIT {}",
            vector_to_string(&query_embedding),
            vector_to_string(&query_embedding),
            min_similarity,
            vector_to_string(&query_embedding),
            limit
        );
        
        // Execute raw SQL (pgvector operations)
        diesel::sql_query(&sql)
            .load::<(i64, String, f32)>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, (page_id, preview, similarity))| SearchResult {
                        page_id,
                        content_preview: preview,
                        similarity,
                        rank: (idx + 1) as i32,
                    })
                    .collect()
            })
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let total = results.len();
    
    Ok(HttpResponse::Ok().json(SearchResponse {
        query,
        results,
        total,
        search_type: "semantic".to_string(),
    }))
}

/// Generate content hash for deduplication
fn generate_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generate embedding vector using OpenAI
async fn generate_embedding_vector(text: &str) -> Result<Embedding, CustomHttpError> {
    #[derive(Serialize)]
    struct EmbeddingRequest {
        input: String,
        model: String,
    }
    
    #[derive(Deserialize)]
    struct EmbeddingResponse {
        data: Vec<EmbeddingData>,
    }
    
    #[derive(Deserialize)]
    struct EmbeddingData {
        embedding: Vec<f32>,
    }
    
    let client = Client::new();
    
    // TODO: Get API key from provider config
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "sk-test".to_string());
    
    let request = EmbeddingRequest {
        input: text.to_string(),
        model: "text-embedding-ada-002".to_string(),
    };
    
    let response = client
        .post("https://api.openai.com/v1/embeddings")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Embedding request failed: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(CustomHttpError::InternalServerError("Embedding generation failed".to_string()));
    }
    
    let embedding_response: EmbeddingResponse = response
        .json()
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Parse error: {}", e)))?;
    
    embedding_response
        .data
        .first()
        .map(|d| d.embedding.clone())
        .ok_or_else(|| CustomHttpError::InternalServerError("No embedding in response".to_string()))
}

/// Convert vector to PostgreSQL array string
fn vector_to_string(vec: &[f32]) -> String {
    vec.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
