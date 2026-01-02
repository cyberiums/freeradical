use actix_web::{post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::services::seo_auditor::SEOAuditor;

#[derive(Deserialize, ToSchema)]
pub struct AuditRequest {
    pub url: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuditError {
    pub error: String,
}

/// Perform an SEO audit for a URL
#[utoipa::path(
    post,
    path = "/admin/seo/audit",
    tag = "Internal - SEO",
    request_body = AuditRequest,
    responses(
        (status = 200, description = "SEO audit results"),
        (status = 400, description = "Invalid URL")
    )
)]
#[post("/admin/seo/audit")]
pub async fn audit_url(payload: web::Json<AuditRequest>) -> impl Responder {
    let url = &payload.url;
    
    // Validate URL
    if !url.starts_with("http") {
        return HttpResponse::BadRequest().json(AuditError {
            error: "URL must start with http:// or https://".to_string(),
        });
    }

    // Fetch content
    // Note: Using reqwest here. Assuming it's available in dependencies.
    let content = match reqwest::get(url).await {
        Ok(res) => match res.text().await {
            Ok(txt) => txt,
            Err(e) => return HttpResponse::InternalServerError().json(AuditError {
                error: format!("Failed to read content: {}", e),
            }),
        },
        Err(e) => return HttpResponse::InternalServerError().json(AuditError {
            error: format!("Failed to fetch URL: {}", e),
        }),
    };

    // Run Audit
    let auditor = SEOAuditor;
    let result = auditor.audit(url, &content);

    HttpResponse::Ok().json(result)
}
