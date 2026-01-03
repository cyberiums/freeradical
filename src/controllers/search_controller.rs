// Search Controller - Temporarily disabled
use actix_web::{web, HttpResponse, Responder, get};
use utoipa::ToSchema;
use crate::models::DatabasePool;

/// Search across pages, modules, and media
#[utoipa::path(
    get,
    path = "/search",
    tag = "Content - Search",
    params(
        ("q" = String, Query, description = "Search query"),
        ("resources" = Option<String>, Query, description = "Resources to search"),
        ("page" = Option<i32>, Query, description = "Page number"),
        ("per_page" = Option<i32>, Query, description = "Results per page")
    ),
    responses(
        (status = 501, description = "Search temporarily disabled")
    )
)]
#[get("/search")]
pub async fn search_content(
    _query: web::Query<serde_json::Value>,
    _pool: web::Data<DatabasePool>
) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
   "error": "Search functionality temporarily disabled"
    }))
}
