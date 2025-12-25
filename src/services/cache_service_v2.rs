// Cache Service V2 - Redis-based caching with connection pooling

use deadpool_redis::{Config, Runtime, Pool, redis::AsyncCommands};
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;

pub struct CacheServiceV2 {
    pool: Pool,
    default_ttl: usize,
}

impl CacheServiceV2 {
    /// Create new cache service from Redis URL
    pub async fn new(redis_url: &str, default_ttl: usize) -> Result<Self, Box<dyn Error>> {
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        
        // Test connection
        let mut conn = pool.get().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        
        log::info!("✅ Redis cache connected: {}", redis_url);
        
        Ok(Self {
            pool,
            default_ttl,
        })
    }
    
    /// Get cached value
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.pool.get().await.ok()?;
        let value: String = conn.get(key).await.ok()?;
        serde_json::from_str(&value).ok()
    }
    
    /// Set cached value with optional TTL
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<usize>
    ) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let serialized = serde_json::to_string(value)?;
        let ttl = ttl.unwrap_or(self.default_ttl);
        
        conn.set_ex(key, serialized, ttl).await?;
        Ok(())
    }
    
    /// Delete single cache entry
    pub async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        conn.del(key).await?;
        Ok(())
    }
    
    /// Delete all keys matching pattern (e.g., "pages:*")
    pub async fn delete_pattern(&self, pattern: &str) -> Result<usize, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let keys: Vec<String> = conn.keys(pattern).await?;
        
        if keys.is_empty() {
            return Ok(0);
        }
        
        let count = keys.len();
        conn.del(&keys).await?;
        
        log::debug!("🗑️ Deleted {} cache keys matching '{}'", count, pattern);
        Ok(count)
    }
    
    /// Increment counter (useful for rate limiting)
    pub async fn incr(&self, key: &str) -> Result<i64, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        let count: i64 = conn.incr(key, 1).await?;
        Ok(count)
    }
    
    /// Set expiration on existing key
    pub async fn expire(&self, key: &str, seconds: usize) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get().await?;
        conn.expire(key, seconds as i64).await?;
        Ok(())
    }
    
    /// Check if key exists
    pub async fn exists(&self, key: &str) -> bool {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(_) => return false,
        };
        
        conn.exists(key).await.unwrap_or(false)
    }
    
    /// Get remaining TTL for a key
    pub async fn ttl(&self, key: &str) -> Option<i64> {
        let mut conn = self.pool.get().await.ok()?;
        conn.ttl(key).await.ok()
    }
}
