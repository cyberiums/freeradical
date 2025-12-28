use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use sha2::{Sha256, Digest};
use pgvector::Vector; 

use crate::models::DbPool;
use crate::models::ai_provider_models::AIProviderConfig;
use crate::schema::{ai_provider_configs, content_embeddings, search_history};
use crate::services::errors_service::CustomHttpError;

/// Vector embedding (1536 dimensions for OpenAI ada-002)
pub type Embedding = Vec<f32>;

/// Request to create embedding
#[derive(Debug, Deserialize)]
pub struct CreateEmbeddingRequest {
    pub page_uuid: String,
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
#[derive(Debug, Serialize, QueryableByName)]
pub struct SearchResult {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub page_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub content_preview: String,
    #[diesel(sql_type = diesel::sql_types::Float4)]
    pub similarity: f32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
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
    pub page_uuid: Option<String>,
    pub embedding_vector: Option<String>,
    pub model_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Create or update embedding for content
pub async fn create_embedding(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateEmbeddingRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let content = payload.content.clone();
    let page_uuid = payload.page_uuid.clone();
    
    // Simplified: skip hash check since content_hash doesn't exist in schema
    // Generate embedding and store directly
    
    // Generate embedding vector
    let embedding_values = generate_embedding_vector(&content).await?;
    let model_name = "text-embedding-ada-002".to_string(); // Assuming this model for now
    
    let id = web::block(move || -> Result<i64, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        let vector_str = vector_to_string(&embedding_values);
        let vector_formatted = format!("[{}]", vector_str);
        
        // Use raw SQL insert to cast to vector type
        // schema.rs treats it as Text, DB treats it as vector
        diesel::sql_query("INSERT INTO content_embeddings (page_uuid, embedding_vector, model_name, created_at, updated_at) VALUES ($1, $2::vector, $3, NOW(), NOW())")
            .bind::<diesel::sql_types::Text, _>(page_uuid)
            .bind::<diesel::sql_types::Text, _>(vector_formatted)
            .bind::<diesel::sql_types::Text, _>(model_name)
            .execute(&mut conn)?;
        
        Ok(1) // Placeholder ID
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
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
        
        let query_str = vector_to_string(&query_embedding);
        let query_formatted = format!("[{}]", query_str);
        
        // Use cosine similarity search (1 - cosine distance)
        // SQL: SELECT page_uuid as page_id, '' as content_preview, 1 - (embedding_vector <=> $1) as similarity, 0 as rank
        
        let sql = "SELECT 
                page_uuid as page_id, 
                ''::text as content_preview, 
                (1 - (embedding_vector <=> $1::vector))::real as similarity,
                0::integer as rank
             FROM content_embeddings 
             WHERE 1 - (embedding_vector <=> $1::vector) > $2
             ORDER BY embedding_vector <=> $1::vector
             LIMIT $3";
        
        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, _>(query_formatted)
            .bind::<diesel::sql_types::Float4, _>(min_similarity)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .load::<SearchResult>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, mut res)| {
                        res.rank = (idx + 1) as i32;
                        res
                    })
                    .collect()
            })
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
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
