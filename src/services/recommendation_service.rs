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
        
        // Note: pages table uses uuid as PK, not id
        // SQL simplified without JOIN since embedding doesn't exist in schema
        // TODO: Implement proper vector similarity when pgvector is set up
        let recommendations: Vec<Recommendation> = vec![];
        
        Ok(recommendations)
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
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
            .select((pages::uuid, pages::page_title, pages::time_created))
            .filter(pages::status.eq("published"))
            .order(pages::time_created.desc())
            .limit(limit)
            .load::<(String, String, chrono::NaiveDateTime)>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, (page_uuid, title, _))| Recommendation {
                        page_id: idx as i64, // Using index as placeholder ID
                        title,
                        score: 1.0 - (idx as f32 * 0.1),
                        rank: (idx + 1) as i32,
                        reason: "Trending".to_string(),
                    })
                    .collect()
            })
    })
    .await.map_err(|e| CustomHttpError::InternalServerError(format!("Operation failed: {}", e)))?
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
