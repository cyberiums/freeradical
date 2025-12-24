use actix_web::{HttpResponse, Responder};
use diesel::prelude::*;

use crate::services::database_service;

/// Generate image sitemap XML
/// Standardized to manual routing (no macro)
pub async fn image_sitemap() -> impl Responder {
    use crate::schema::pages::dsl::*;
    
    let mut conn = database_service::establish_connection();
    
    // Get all pages with featured images
    let results = pages
        .select((page_url, featured_image, time_created))
        .filter(featured_image.is_not_null())
        .load::<(String, Option<String>, chrono::NaiveDateTime)>(&mut conn);
    
    match results {
        Ok(page_list) => {
            let base_url = std::env::var("APP_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
            
            let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
"#);
            
            for (url, img, _) in page_list {
                if let Some(image_url) = img {
                    sitemap.push_str(&format!(r#"  <url>
    <loc>{}{}</loc>
    <image:image>
      <image:loc>{}</image:loc>
    </image:image>
  </url>
"#, base_url, url, image_url));
                }
            }
            
            sitemap.push_str("</urlset>");
            
            HttpResponse::Ok()
                .content_type("application/xml; charset=utf-8")
                .body(sitemap)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error generating image sitemap"),
    }
}
