use sha2::{Sha256, Digest};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db_connection::establish_connection;

/// Analytics tracking service
/// Privacy-compliant with IP hashing
pub struct AnalyticsService;

impl AnalyticsService {
    /// Track a page view
    /// Non-blocking - errors are logged but don't affect response
    pub fn track_page_view(
        page_url: &str,
        page_uuid: Option<&str>,
        ip_address: &str,
        referrer: Option<&str>,
        user_agent: Option<&str>,
    ) {
        // Hash IP for privacy
        let visitor_hash = Self::hash_ip(ip_address);
        
        // Insert page view asynchronously (fire and forget)
        std::thread::spawn(move || {
            if let Err(e) = Self::insert_page_view(
                page_url.to_string(),
                page_uuid.map(|s| s.to_string()),
                visitor_hash,
                referrer.map(|s| s.to_string()),
                user_agent.map(|s| s.to_string()),
            ) {
                eprintln!("Analytics error: {}", e);
            }
        });
    }
    
    /// Hash IP address for privacy
    fn hash_ip(ip: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ip.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Insert page view into database
    fn insert_page_view(
        page_url: String,
        page_uuid: Option<String>,
        visitor_hash: String,
        referrer: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::page_views;
        
        let mut conn = establish_connection();
        
        diesel::insert_into(page_views::table)
            .values((
                page_views::page_url.eq(&page_url),
                page_views::page_uuid.eq(&page_uuid),
                page_views::visitor_hash.eq(&visitor_hash),
                page_views::referrer.eq(&referrer),
                page_views::user_agent.eq(&user_agent),
            ))
            .execute(&mut conn)?;
        
        Ok(())
    }
    
    /// Get page view count for a URL
    pub fn get_page_views(page_url: &str) -> i64 {
        use crate::schema::page_views::dsl;
        
        let mut conn = establish_connection();
        
        dsl::page_views
            .filter(dsl::page_url.eq(page_url))
            .count()
            .get_result(&mut conn)
            .unwrap_or(0)
    }
    
    /// Get top pages by views
    pub fn get_top_pages(limit: i64) -> Vec<(String, i64)> {
        use crate::schema::page_views::dsl;
        
        let mut conn = establish_connection();
        
        dsl::page_views
            .group_by(dsl::page_url)
            .select((dsl::page_url, diesel::dsl::count(dsl::id)))
            .order(diesel::dsl::count(dsl::id).desc())
            .limit(limit)
            .load::<(String, i64)>(&mut conn)
            .unwrap_or_else(|_| vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ip_hashing() {
        let hash1 = AnalyticsService::hash_ip("192.168.1.1");
        let hash2 = AnalyticsService::hash_ip("192.168.1.1");
        let hash3 = AnalyticsService::hash_ip("192.168.1.2");
        
        assert_eq!(hash1, hash2); // Same IP = same hash
        assert_ne!(hash1, hash3); // Different IP = different hash
        assert_eq!(hash1.len(), 64); // SHA256 = 64 hex chars
    }
}
