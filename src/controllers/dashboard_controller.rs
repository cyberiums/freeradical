use actix_web::{get, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};
use crate::services::analytics_service::AnalyticsService;
use crate::db_connection::establish_connection;
use diesel::prelude::*;

#[derive(Serialize)]
struct DashboardSummary {
    total_pages: i64,
    total_views_today: i64,
    total_views_week: i64,
    total_views_all_time: i64,
    top_pages: Vec<TopPage>,
}

#[derive(Serialize)]
struct TopPage {
    url: String,
    views: i64,
}

#[derive(Serialize)]
struct AnalyticsSummary {
    views_today: i64,
    views_week: i64,
    views_month: i64,
    unique_visitors_today: i64,
    top_referrers: Vec<String>,
}

#[derive(Serialize)]
struct SEOHealthCheck {
    total_pages: i64,
    pages_with_meta_description: i64,
    pages_with_canonical_url: i64,
    pages_with_og_image: i64,
    seo_score: f64,
    issues: Vec<String>,
}

/// GET /admin/dashboard/summary
/// Main dashboard summary with key metrics
#[get("/admin/dashboard/summary")]
pub async fn dashboard_summary() -> impl Responder {
    use crate::schema::pages::dsl::*;
    
    let mut conn = establish_connection();
    
    // Get total pages
    let total_pages_count = pages.count().get_result::<i64>(&mut conn).unwrap_or(0);
    
    // Get top pages (simplified - using all-time views)
    let top_pages_data = AnalyticsService::get_top_pages(10)
        .into_iter()
        .map(|(url, views)| TopPage { url, views })
        .collect();
    
    let summary = DashboardSummary {
        total_pages: total_pages_count,
        total_views_today: 0, // TODO: Implement time-based queries
        total_views_week: 0,
        total_views_all_time: 0,
        top_pages: top_pages_data,
    };
    
    HttpResponse::Ok().json(summary)
}

/// GET /admin/analytics/summary
/// Detailed analytics summary
#[get("/admin/analytics/summary")]
pub async fn analytics_summary() -> impl Responder {
    let summary = AnalyticsSummary {
        views_today: 0,
        views_week: 0,
        views_month: 0,
        unique_visitors_today: 0,
        top_referrers: vec![],
    };
    
    HttpResponse::Ok().json(summary)
}

/// GET /admin/seo/health
/// SEO health check
#[get("/admin/seo/health")]
pub async fn seo_health() -> impl Responder {
    use crate::schema::pages::dsl::*;
    
    let mut conn = establish_connection();
    
    let total = pages.count().get_result::<i64>(&mut conn).unwrap_or(0);
    
    let with_meta_desc = pages
        .filter(meta_description.is_not_null())
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);
    
    let with_canonical = pages
        .filter(canonical_url.is_not_null())
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);
    
    let with_og_image = pages
        .filter(og_image.is_not_null())
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);
    
    let mut issues = vec![];
    if with_meta_desc < total {
        issues.push(format!("{} pages missing meta description", total - with_meta_desc));
    }
    if with_canonical < total {
        issues.push(format!("{} pages missing canonical URL", total - with_canonical));
    }
    
    // Calculate SEO score (0-100)
    let score = if total > 0 {
        let meta_score = (with_meta_desc as f64 / total as f64) * 40.0;
        let canonical_score = (with_canonical as f64 / total as f64) * 30.0;
        let og_score = (with_og_image as f64 / total as f64) * 30.0;
        meta_score + canonical_score + og_score
    } else {
        0.0
    };
    
    let health = SEOHealthCheck {
        total_pages: total,
        pages_with_meta_description: with_meta_desc,
        pages_with_canonical_url: with_canonical,
        pages_with_og_image: with_og_image,
        seo_score: score,
        issues,
    };
    
    HttpResponse::Ok().json(health)
}

/// GET /admin/analytics/pages
/// Top pages by views
#[get("/admin/analytics/pages")]
pub async fn top_pages() -> impl Responder {
    let pages = AnalyticsService::get_top_pages(20)
        .into_iter()
        .map(|(url, views)| TopPage { url, views })
        .collect::<Vec<_>>();
    
    HttpResponse::Ok().json(pages)
}
