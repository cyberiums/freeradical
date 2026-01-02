use actix_web::{get, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::services::analytics_service::AnalyticsService;
use crate::services::database_service::establish_connection;
use diesel::prelude::*;

#[derive(Serialize, ToSchema)]
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

/// Main dashboard summary with key metrics
#[utoipa::path(
    get,
    path = "/admin/dashboard/summary",
    tag = "Internal - Dashboard",
    responses(
        (status = 200, description = "Dashboard summary", body = DashboardSummary)
    )
)]
#[get("/admin/dashboard/summary")]
pub async fn dashboard_summary() -> impl Responder {
    use crate::schema::pages::dsl::*;
    
    let mut conn = establish_connection();
    
    // Get total pages
    let total_pages_count = pages.count().get_result::<i64>(&mut conn).unwrap_or(0);
    
    // Get time-based views
    let views_today = AnalyticsService::get_views_today();
    let views_week = AnalyticsService::get_views_week();
    let views_all_time = AnalyticsService::get_views_month(); // Using month as proxy for all-time
    
    // Get top pages
    let top_pages_data = AnalyticsService::get_top_pages(10)
        .into_iter()
        .map(|(url, views)| TopPage { url, views })
        .collect();
    
    let summary = DashboardSummary {
        total_pages: total_pages_count,
        total_views_today: views_today,
        total_views_week: views_week,
        total_views_all_time: views_all_time,
        top_pages: top_pages_data,
    };
    
    HttpResponse::Ok().json(summary)
}

/// Detailed analytics summary
#[utoipa::path(
    get,
    path = "/admin/analytics/summary",
    tag = "Internal - Dashboard",
    responses(
        (status = 200, description = "Analytics summary", body = AnalyticsSummary)
    )
)]
#[get("/admin/analytics/summary")]
pub async fn analytics_summary() -> impl Responder {
    let views_today = AnalyticsService::get_views_today();
    let views_week = AnalyticsService::get_views_week();
    let views_month = AnalyticsService::get_views_month();
    let unique_today = AnalyticsService::get_unique_visitors_today();
    
    let top_refs = AnalyticsService::get_top_referrers(10)
        .into_iter()
        .map(|(url, _)| url)
        .collect();
    
    let summary = AnalyticsSummary {
        views_today,
        views_week,
        views_month,
        unique_visitors_today: unique_today,
        top_referrers: top_refs,
    };
    
    HttpResponse::Ok().json(summary)
}

/// SEO health check
#[utoipa::path(
    get,
    path = "/admin/seo/health",
    tag = "Internal - Dashboard",
    responses(
        (status = 200, description = "SEO health check", body = SEOHealthCheck)
    )
)]
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

/// Top pages by views
#[utoipa::path(
    get,
    path = "/admin/analytics/pages",
    tag = "Internal - Dashboard",
    responses(
        (status = 200, description = "Top pages by views")
    )
)]
#[get("/admin/analytics/pages")]
pub async fn top_pages() -> impl Responder {
    let pages = AnalyticsService::get_top_pages(20)
        .into_iter()
        .map(|(url, views)| TopPage { url, views })
        .collect::<Vec<_>>();
    
    HttpResponse::Ok().json(pages)
}
