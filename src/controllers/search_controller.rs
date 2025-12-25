// Search Controller
use actix_web::{web, HttpResponse, Responder, get};
use crate::services::search_service::{SearchQuery, search};
use crate::models::{DatabasePool, pool_handler};

/// Search across pages, modules, and media
/// GET /api/search?q=query&resources=pages,modules&page=1&per_page=20
#[get("/search")]
pub async fn search_content(
    query: web::Query<SearchQuery>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let mut conn = match pool_handler(pool) {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database connection error"),
    };
    
    match search(&query.into_inner(), &mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => HttpResponse::InternalServerError().json(format!("Search error: {}", e)),
    }
}
