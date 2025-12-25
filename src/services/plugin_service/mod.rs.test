use actix_web::{dev::ServiceRequest, HttpResponse};
use std::any::Any;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub mod middleware;

/// The core Plugin trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync + Any {
    /// Unique identifier for the plugin
    fn name(&self) -> &str;
    
    /// Version string (semver)
    fn version(&self) -> &str;
    
    /// Called when the application starts
    async fn on_load(&self) -> Result<(), String> {
        Ok(())
    }
    
    /// Called on every incoming HTTP request (middleware-like)
    async fn on_request(&self, _req: &ServiceRequest) -> Result<(), String> {
        Ok(())
    }
    
    /// Called before content is saved to database
    async fn on_save_content(&self, content_type: &str, data: &serde_json::Value) -> Result<(), String> {
        Ok(())
    }
}

/// Registry to manage loaded plugins
pub struct PluginRegistry {
    plugins: Arc<Mutex<Vec<Arc<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a new plugin
    pub fn register(&self, plugin: Arc<dyn Plugin>) {
        let mut plugins = self.plugins.lock().unwrap();
        log::info!("Registering plugin: {} v{}", plugin.name(), plugin.version());
        plugins.push(plugin);
    }

    /// Execute on_load hooks for all plugins
    pub async fn load_all(&self) {
        let plugins: Vec<Arc<dyn Plugin>> = {
            let guard = self.plugins.lock().unwrap();
            guard.clone()
        };
        
        log::info!("Loading {} plugins", plugins.len());
        for plugin in plugins {
             let _ = plugin.on_load().await;
        }
    }

    pub async fn execute_on_request(&self, req: &ServiceRequest) {
        let plugins: Vec<Arc<dyn Plugin>> = {
             let guard = self.plugins.lock().unwrap();
             guard.clone()
        };
        
        for plugin in plugins {
             let _ = plugin.on_request(req).await;
        }
    }
    
    pub fn get_hooks(&self) -> Vec<String> {
        let plugins = self.plugins.lock().unwrap();
        plugins.iter().map(|p| p.name().to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test-plugin"
        }
        
        fn version(&self) -> &str {
            "0.1.0"
        }
    }

    #[actix_rt::test]
    async fn test_plugin_registry() {
        let registry = PluginRegistry::new();
        registry.register(Arc::new(TestPlugin));
        
        let hooks = registry.get_hooks();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0], "test-plugin");
        
        registry.load_all().await;
    }
}
