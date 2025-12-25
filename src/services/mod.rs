pub mod errors_service;
pub mod cache_service;
pub mod analytics_service;
pub mod auth_service;
pub mod database_service;
pub mod revision_service;
pub mod inventory_service;
pub mod ai_provider_service;  // AI provider management
pub mod ai_content_service;  // AI content generation
pub mod scheduler_service;
pub mod monitoring_service;
pub mod field_validation_service;
pub mod permission_service;
pub mod search_service;
pub mod cache_service_v2;
pub mod webhook_service;
pub mod image_service;
pub mod plugin_service;
pub mod template_service;
pub mod totp_service;
pub mod backup_service;
pub mod payment_service;
// Services below require migrations to be run first
// pub mod language_service;
// pub mod analytics_service_v2;
// pub mod oauth_service;
// pub mod hreflang_service;