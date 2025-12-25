use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::models::DbPool;
use crate::schema::{pages, content_embeddings};
use crate::services::errors_service::CustomHttpError;

/// Request for recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationRequest {
    pub page_id: i64,
    pub limit: Option<i64>,
    pub recommendation_type: Option<String>, // 'similar', 'related', 'personalized'
}

/// Recommendation result
#[derive(Debug, Serialize)]
pub struct Recommendation {
    pub page_id: i64,
    pub title: String,
    pub score: f32,
    pub rank: i32,
    pub reason: String,
}

/// Recommendation response
#[derive(Debug, Serialize)]
pub struct RecommendationResponse {
    pub source_page_id: i64,
    pub recommendations: Vec<Recommendation>,
    pub total: usize,
    pub algorithm: String,
}

/// Get related content recommendations
pub async fn get_related_content(
    pool: web::Data<DbPool>,
    payload: web::Json<RecommendationRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let source_id = payload.page_id;
    let limit = payload.limit.unwrap_or(5);
    
    // Get similar content using vector similarity
    let recommendations = web::block(move || -> Result<Vec<Recommendation>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        // Get source embedding
        let source_embedding: Vec<f32> = content_embeddings::table
            .filter(content_embeddings::page_id.eq(source_id))
            .select(content_embeddings::embedding)
            .first(&mut conn)?;
        
        // Find similar content
        let sql = format!(
            "SELECT ce.page_id, p.title, 1 - (ce.embedding <=> '[{}]') AS score 
             FROM content_embeddings ce
             JOIN pages p ON p.id = ce.page_id
             WHERE ce.page_id != {}
             ORDER BY ce.embedding <=> '[{}]'
             LIMIT {}",
            vector_to_string(&source_embedding),
            source_id,
            vector_to_string(&source_embedding),
            limit
        );
        
        diesel::sql_query(&sql)
            .load::<(i64, String, f32)>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, (page_id, title, score))| Recommendation {
                        page_id,
                        title,
                        score,
                        rank: (idx + 1) as i32,
                        reason: "Content similarity".to_string(),
                    })
                    .collect()
            })
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    let total = recommendations.len();
    
    Ok(HttpResponse::Ok().json(RecommendationResponse {
        source_page_id: source_id,
        recommendations,
        total,
        algorithm: "collaborative_filtering".to_string(),
    }))
}

/// Get trending content
pub async fn get_trending(
    pool: web::Data<DbPool>,
    limit: web::Query<i64>,
) -> Result<HttpResponse, CustomHttpError> {
    let limit = limit.into_inner();
    
    // TODO: Implement view tracking and calculate trending
    // For now, return most recent content
    let trending = web::block(move || -> Result<Vec<Recommendation>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        pages::table
            .select((pages::id, pages::title, pages::created_at))
            .filter(pages::status.eq("published"))
            .order(pages::created_at.desc())
            .limit(limit)
            .load::<(i64, String, chrono::NaiveDateTime)>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, (page_id, title, _))| Recommendation {
                        page_id,
                        title,
                        score: 1.0 - (idx as f32 * 0.1),
                        rank: (idx + 1) as i32,
                        reason: "Trending".to_string(),
                    })
                    .collect()
            })
    })
    .await?
    .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "trending": trending,
        "algorithm": "recency_based"
    })))
}

/// Convert vector to PostgreSQL array string
fn vector_to_string(vec: &[f32]) -> String {
    vec.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
