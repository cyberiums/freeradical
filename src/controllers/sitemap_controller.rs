use actix_web::{HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

use crate::services::database_service;

/// Generate XML sitemap with gzip compression support
#[utoipa::path(
    get,
    path = "/sitemap.xml",
    tag = "Content - SEO",
    responses(
        (status = 200, description = "XML sitemap (optionally gzipped)", content_type = "application/xml"),
        (status = 500, description = "Error generating sitemap")
    )
)]
pub async fn sitemap(req: HttpRequest) -> impl Responder {
    use crate::schema::pages::dsl::*;
    
    let mut conn = database_service::establish_connection();
    
    // Get all pages
    let results = pages
        .select((page_url, time_created))
        .load::<(String, chrono::NaiveDateTime)>(&mut conn);
    
    match results {
        Ok(page_list) => {
            let base_url = std::env::var("APP_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
            
            let page_count = page_list.len();
            
            // If >50k URLs, generate sitemap index (future enhancement)
            // For now, generate single sitemap
            
            let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);
            
            for (url, last_mod) in page_list {
                let formatted_date = last_mod.format("%Y-%m-%d").to_string();
                sitemap.push_str(&format!(
                    r#"  <url>
    <loc>{}{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
                    base_url, url, formatted_date
                ));
            }
            
            sitemap.push_str("</urlset>");
            
            // Check if client supports gzip
            let accepts_gzip = req
                .headers()
                .get("accept-encoding")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.contains("gzip"))
                .unwrap_or(false);
            
            if accepts_gzip {
                // Return gzipped sitemap
                match compress_sitemap(&sitemap) {
                    Ok(compressed) => HttpResponse::Ok()
                        .content_type("application/xml; charset=utf-8")
                        .set_header("Content-Encoding", "gzip")
                        .set_header("X-Sitemap-Count", page_count.to_string())
                        .body(compressed),
                    Err(_) => {
                        // Fallback to uncompressed
                        HttpResponse::Ok()
                            .content_type("application/xml; charset=utf-8")
                            .set_header("X-Sitemap-Count", page_count.to_string())
                            .body(sitemap)
                    }
                }
            } else {
                // Return uncompressed sitemap
                HttpResponse::Ok()
                    .content_type("application/xml; charset=utf-8")
                    .set_header("X-Sitemap-Count", page_count.to_string())
                    .body(sitemap)
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error generating sitemap"),
    }
}

/// Compress sitemap with gzip
fn compress_sitemap(xml: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(xml.as_bytes())?;
    encoder.finish()
}
