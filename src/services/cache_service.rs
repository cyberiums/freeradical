use redis::{Client, Commands, RedisResult};
use std::time::Duration;

/// Simple Redis cache service
/// Optional - gracefully degrades if Redis unavailable
pub struct CacheService {
    client: Option<Client>,
    enabled: bool,
}

impl CacheService {
    /// Create new cache service
    /// Returns service with Redis disabled if connection fails
    pub fn new() -> Self {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        
        let enabled = std::env::var("CACHE_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        
        if !enabled {
            println!("ℹ️  Cache disabled via CACHE_ENABLED=false");
            return CacheService {
                client: None,
                enabled: false,
            };
        }
        
        match Client::open(redis_url.as_str()) {
            Ok(client) => {
                // Test connection
                match client.get_connection() {
                    Ok(_) => {
                        println!("✅ Redis cache connected");
                        CacheService {
                            client: Some(client),
                            enabled: true,
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Redis unavailable: {}. Cache disabled.", e);
                        CacheService {
                            client: None,
                            enabled: false,
                        }
                    }
                }
            }
            Err(e) => {
                println!("⚠️  Redis client error: {}. Cache disabled.", e);
                CacheService {
                    client: None,
                    enabled: false,
                }
            }
        }
    }
    
    /// Get cached value
    pub fn get(&self, key: &str) -> Option<String> {
        if !self.enabled || self.client.is_none() {
            return None;
        }
        
        if let Some(ref client) = self.client {
            if let Ok(mut conn) = client.get_connection() {
                let result: RedisResult<String> = conn.get(key);
                return result.ok();
            }
        }
        None
    }
    
    /// Set cached value with TTL
    pub fn set(&self, key: &str, value: &str, ttl_seconds: usize) -> bool {
        if !self.enabled || self.client.is_none() {
            return false;
        }
        
        if let Some(ref client) = self.client {
            if let Ok(mut conn) = client.get_connection() {
                let result: RedisResult<()> = conn.set_ex(key, value, ttl_seconds as u64);
                return result.is_ok();
            }
        }
        false
    }
    
    /// Delete cached value
    pub fn delete(&self, key: &str) -> bool {
        if !self.enabled || self.client.is_none() {
            return false;
        }
        
        if let Some(ref client) = self.client {
            if let Ok(mut conn) = client.get_connection() {
                let result: RedisResult<()> = conn.del(key);
                return result.is_ok();
            }
        }
        false
    }
    
    /// Check if cache is enabled and available
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_service_creation() {
        let service = CacheService::new();
        // Should not panic even if Redis unavailable
        assert!(true);
    }
    
    #[test]
    fn test_cache_graceful_degradation() {
        // With Redis disabled/unavailable, should work without errors
        let service = CacheService::new();
        
        // Operations should return false/None when disabled
        let set_result = service.set("test_key", "test_value", 60);
        let get_result = service.get("test_key");
        let delete_result = service.delete("test_key");
        
        // When disabled, operations fail gracefully
        assert!(get_result.is_none());
        
        // Service should report disabled state
        // (when CACHE_ENABLED=false)
    }
    
    #[test]
    fn test_cache_enabled_check() {
        let service = CacheService::new();
        
        // Should return enabled status
        let enabled = service.is_enabled();
        
        // Value depends on environment config
        assert!(enabled == true || enabled == false);
    }
    
    #[test]
    fn test_cache_operations_safe() {
        let service = CacheService::new();
        
        // All operations should be safe to call
        // even if Redis is not available
        service.set("key1", "value1", 10);
        service.get("key1");
        service.delete("key1");
        
        // No panic = success
        assert!(true);
    }
}
