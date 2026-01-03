// Cache Service V2 - Redis-based caching with connection pooling (Single + Cluster)

use deadpool_redis::{Config, Runtime, Pool};
use redis::{cluster::ClusterClient, AsyncCommands};
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;

#[derive(Clone)]
pub enum RedisBackend {
    Single(Pool),
    Cluster(ClusterClient),
}

#[derive(Clone)]
pub struct CacheServiceV2 {
    backend: RedisBackend,
    default_ttl: usize,
}

impl CacheServiceV2 {
    /// Create new cache service.
    /// Prioritizes Cluster configuration if `cluster_nodes` is provided.
    pub async fn new(redis_url: &str, cluster_nodes: Option<String>, default_ttl: usize) -> Result<Self, Box<dyn Error>> {
        if let Some(nodes_str) = cluster_nodes {
            if !nodes_str.is_empty() {
                log::info!("Initializing Redis Cluster...");
                let nodes: Vec<&str> = nodes_str.split(',').collect();
                let client = ClusterClient::new(nodes)?;
                
                // Test connection
                // ClusterClient manages connection internally
                
                return Ok(Self {
                    backend: RedisBackend::Cluster(client),
                    default_ttl
                });
            }
        }
    
        // Fallback to Single Node
        log::info!("Initializing Single Node Redis...");
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        
        // Test connection
        let mut conn = pool.get().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        
        Ok(Self {
            backend: RedisBackend::Single(pool),
            default_ttl,
        })
    }
    
    /// Get cached value
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let value: String = match &self.backend {
            RedisBackend::Single(pool) => {
                let mut conn = pool.get().await.ok()?;
                conn.get(key).await.ok()?
            },
            RedisBackend::Cluster(client) => {
                let mut conn = client.get_async_connection().await.ok()?;
                conn.get(key).await.ok()?
            }
        };

        serde_json::from_str(&value).ok()
    }
    
    /// Set cached value with optional TTL
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<usize>
    ) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(value)?;
        let ttl = ttl.unwrap_or(self.default_ttl);
        
        match &self.backend {
            RedisBackend::Single(pool) => {
                let mut conn = pool.get().await?;
                conn.set_ex::<_, _, ()>(key, serialized, ttl as u64).await?;
            },
            RedisBackend::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.set_ex::<_, _, ()>(key, serialized, ttl as u64).await?;
            }
        }

        Ok(())
    }
    
    /// Delete single cache entry
    pub async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        match &self.backend {
             RedisBackend::Single(pool) => {
                let mut conn = pool.get().await?;
                conn.del::<_, ()>(key).await?;
             },
             RedisBackend::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.del::<_, ()>(key).await?;
             }
        }
        Ok(())
    }

    // Note: delete_pattern is complex in Cluster (scan across nodes). 
    // Simplified: Only support pattern deletion in Single node mode for now, or log warning.
    
    /// Delete all keys matching pattern (e.g., "pages:*"). 
    /// WARNING: Pattern scan is expensive in Cluster.
    pub async fn delete_pattern(&self, pattern: &str) -> Result<usize, Box<dyn Error>> {
         match &self.backend {
             RedisBackend::Single(pool) => {
                let mut conn = pool.get().await?;
                let keys: Vec<String> = conn.keys(pattern).await?;
                
                if keys.is_empty() {
                    return Ok(0);
                }
                
                let count = keys.len();
                conn.del::<_, ()>(&keys).await?;
                log::debug!("🗑️ Deleted {} cache keys matching '{}'", count, pattern);
                Ok(count)
             },
             RedisBackend::Cluster(_client) => {
                 // Creating an iterator across all master nodes is required.
                 // For now, return 0 and log warning.
                 log::warn!("delete_pattern not fully supported in Cluster mode yet without cross-slot scanning.");
                 Ok(0)
             }
         }
    }

    /// Check if key exists
    pub async fn exists(&self, key: &str) -> bool {
        match &self.backend {
            RedisBackend::Single(pool) => {
                let mut conn = match pool.get().await {
                    Ok(c) => c,
                    Err(_) => return false,
                };
                conn.exists(key).await.unwrap_or(false)
            },
            RedisBackend::Cluster(client) => {
                 let mut conn = match client.get_async_connection().await {
                    Ok(c) => c,
                    Err(_) => return false,
                };
                conn.exists(key).await.unwrap_or(false)
            }
        }
    }
}
