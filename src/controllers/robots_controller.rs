use actix_web::HttpResponse;

/// GET /robots.txt
/// Returns robots.txt file for search engine crawlers
pub async fn robots_txt() -> HttpResponse {
    let content = r#"User-agent: *
Allow: /
Disallow: /v1/
Disallow: /assets/

Sitemap: http://127.0.0.1:8080/sitemap.xml
"#;
    
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(content)
}
