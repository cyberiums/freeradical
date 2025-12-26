// Search Controller - Temporarily disabled
use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{DatabasePool, pool_handler};

/// Search across pages, modules, and media
/// GET /api/search?q=query&resources=pages,modules&page=1&per_page=20
/// Currently disabled - awaiting search_service implementation
#[get("/search")]
pub async fn search_content(
    _query: web::Query<serde_json::Value>,
    _pool: web::Data<DatabasePool>
) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
   "error": "Search functionality temporarily disabled"
    }))
}
