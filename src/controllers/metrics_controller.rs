use actix_web::{web, HttpResponse, Responder};
use crate::services::monitoring_service::METRICS;

/// Get performance metrics
/// GET /api/metrics
pub async fn get_metrics() -> impl Responder {
    let metrics = METRICS.to_json();
    HttpResponse::Ok().json(metrics)
}

/// Get metrics summary (for health checks)
/// GET /api/health
pub async fn health_check() -> impl Responder {
    let uptime = (chrono::Utc::now() - METRICS.server_start_time).num_seconds();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "uptime_seconds": uptime,
        "features": {
            "media_library": "operational",
            "revision_history": "operational",
            "scheduled_publishing": "operational"
        }
    }))
}
