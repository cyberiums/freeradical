use actix_web::{HttpResponse, web};
use crate::models::{pool_handler, MySQLPool};
use crate::models::page_models::Page;
use crate::models::Model;
use crate::services::errors_service::CustomHttpError;

/// GET /sitemap.xml
/// Generates XML sitemap for all pages
pub async fn sitemap(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    
    // Get all pages from database
    let pages = Page::read_all(&mysql_pool)?;
    
    // Build XML sitemap
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    
    for page in pages {
        xml.push_str("\n  <url>");
        xml.push_str(&format!("\n    <loc>http://127.0.0.1:8080{}</loc>", page.page_url));
        xml.push_str(&format!("\n    <lastmod>{}</lastmod>", page.time_created.format("%Y-%m-%d")));
        xml.push_str("\n    <changefreq>weekly</changefreq>");
        xml.push_str("\n    <priority>0.8</priority>");
        xml.push_str("\n  </url>");
    }
    
    xml.push_str("\n</urlset>");
    
    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml))
}
