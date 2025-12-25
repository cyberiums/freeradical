use actix_web::{get, HttpResponse, Responder};
use crate::services::database_service::establish_connection;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::robots_rules)]
struct RobotRule {
    id: i32,
    user_agent: String,
    directive: String,
    path: String,
    crawl_delay: Option<i32>,
    comment: Option<String>,
    is_active: Option<bool>,  // Nullable in schema
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

/// Generate dynamic robots.txt from database configuration
#[get("/robots.txt")]
pub async fn robots() -> impl Responder {
    use crate::schema::robots_rules::dsl::*;
    
    let mut conn = establish_connection();
    
    // Load active rules from database
    let results = robots_rules
        .filter(is_active.eq(true))
        .order_by(user_agent.asc())
        .load::<RobotRule>(&mut conn);
    
    let base_url = std::env::var("APP_BASE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    
    match results {
        Ok(rules) => {
            let mut robots_txt = String::new();
            let mut current_user_agent = String::new();
            
            for rule in rules {
                // Start new user-agent section if changed
                if rule.user_agent != current_user_agent {
                    if !current_user_agent.is_empty() {
                        robots_txt.push('\n');
                    }
                    robots_txt.push_str(&format!("User-agent: {}\n", rule.user_agent));
                    current_user_agent = rule.user_agent.clone();
                    
                    // Add crawl-delay if specified
                    if let Some(delay) = rule.crawl_delay {
                        robots_txt.push_str(&format!("Crawl-delay: {}\n", delay));
                    }
                }
                
                // Add directive
                if let Some(ref cmt) = rule.comment {
                    robots_txt.push_str(&format!("# {}\n", cmt));
                }
                robots_txt.push_str(&format!("{}: {}\n", rule.directive, rule.path));
            }
            
            // Add sitemap reference
            robots_txt.push_str(&format!("\nSitemap: {}/sitemap.xml\n", base_url));
            robots_txt.push_str(&format!("Sitemap: {}/image-sitemap.xml\n", base_url));
            
            HttpResponse::Ok()
                .content_type("text/plain; charset=utf-8")
                .body(robots_txt)
        }
        Err(_) => {
            // Fallback to default robots.txt if database fails
            let fallback = format!(
                "User-agent: *\nDisallow: /v1/\nDisallow: /admin/\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
                base_url
            );
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(fallback)
        }
    }
}
