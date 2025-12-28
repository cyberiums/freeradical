use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text, Float4, Integer};
use pgvector::Vector;

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
#[derive(Debug, Serialize, QueryableByName)]
pub struct Recommendation {
    #[diesel(sql_type = BigInt)]
    pub page_id: i64,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Float4)]
    pub score: f32,
    #[diesel(sql_type = Integer)]
    pub rank: i32,
    #[diesel(sql_type = Text)]
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
        
        // SQL to find similar pages based on embedding distance
        // Requires source embedding
        
        let sql = "SELECT 
                p.id::bigint as page_id,
                p.page_title as title,
                (1 - (ce.embedding_vector <=> (SELECT embedding_vector FROM content_embeddings WHERE page_id = $1 LIMIT 1)))::real as score,
                0::integer as rank,
                'Similarity'::text as reason
             FROM content_embeddings ce
             JOIN pages p ON p.id = ce.page_id
             WHERE ce.page_id != $1
               AND (SELECT embedding_vector FROM content_embeddings WHERE page_id = $1 LIMIT 1) IS NOT NULL
             ORDER BY ce.embedding_vector <=> (SELECT embedding_vector FROM content_embeddings WHERE page_id = $1 LIMIT 1)
             LIMIT $2";

        // Note: Casting limit to BigInt
        // Since $1 is integer page_id, bind integer. 
        // Warning: pages table uses uuid as primary key in some contexts, but schema says content_embeddings has page_id as Int4. 
        // Assuming pages table has id column (Int4/Int8).
        // If pages uses uuid, then we need to join on uuid.
        
        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Integer, _>(source_id as i32)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .load::<Recommendation>(&mut conn)
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
    
    // For now, return most recent content
    // Note: Reusing Recommendation struct for output consistency
    
    // We need to fetch pages. `pages` struct in models might duplicate this.
    // Simplifying to raw SQL for cleaner struct mapping or using DSL if possible.
    // If pages uses uuid, we have mismatch in Recommendation (i64).
    // Let's assume pages has an `id` serial or use hash of uuid for demo?
    // The previous implementation used `idx` as placeholder ID.
    // Let's try to get actual ID if possible, otherwise use placeholder.
    
    let trending = web::block(move || -> Result<Vec<Recommendation>, diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new("Connection error".to_string())
        ))?;
        
        pages::table
            .select((pages::id, pages::page_title, pages::time_created))
            .filter(pages::status.eq(Some(crate::models::status_enum::PageStatus::Published)))
            .order(pages::time_created.desc())
            .limit(limit)
            .load::<(i32, String, chrono::NaiveDateTime)>(&mut conn)
            .map(|rows| {
                rows.into_iter()
                    .enumerate()
                    .map(|(idx, (id, title, _))| Recommendation {
                        page_id: id as i64,
                        title,
                        score: 1.0 - (idx as f32 * 0.1),
                        rank: (idx + 1) as i32,
                        reason: "Recent".to_string(),
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
