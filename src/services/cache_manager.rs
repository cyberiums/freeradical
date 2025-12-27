use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cache Manager
/// High-performance caching layer
pub struct CacheManager {
    cache: HashMap<String, CachedItem>,
}

/// Cached item
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedItem {
    value: String,
    expires_at: i64,
}

impl CacheManager {
    /// Create new cache manager
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get cached value
    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).map(|item| item.value.clone())
    }

    /// Set cached value
    pub fn set(&mut self, key: String, value: String, ttl_seconds: i64) {
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds;
        self.cache.insert(key, CachedItem { value, expires_at });
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut cache = CacheManager::new();
        cache.set("key".to_string(), "value".to_string(), 3600);
        assert_eq!(cache.get("key"), Some("value".to_string()));
    }
}
