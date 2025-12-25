// Cache Configuration
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub default_ttl: usize,  // seconds
    pub max_pool_size: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            default_ttl: std::env::var("CACHE_TTL")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            max_pool_size: std::env::var("REDIS_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }
}

impl CacheConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
}
