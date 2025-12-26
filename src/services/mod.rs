pub mod errors_service;
pub mod cache_service;
pub mod analytics_service;
pub mod auth_service;
pub mod database_service;
pub mod revision_service;
pub mod inventory_service;

// AI/MCP Services - Phase 0: Security layer only for v1.4.0
pub mod ai_scope_service; // AI security and scoping

// AI Services - Re-enabled for v1.4.0 with models and migrations complete
pub mod ai_provider_service; // Provider CRUD operations
pub mod ai_content_service; // Content generation
pub mod ai_authorization_service; // Budget and permission checks
pub mod semantic_search_service; // Vector search with content_embeddings
pub mod recommendation_service; // Content recommendations

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
pub mod scheduler_service;
// Services below require migrations to be run first
// pub mod language_service;
// pub mod analytics_service_v2;
// pub mod oauth_service;
// pub mod hreflang_service;
pub mod crm_service; // CRM modules below require migrations to be run first