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
pub mod storage_service; // Multi-region storage (S3/Local)
pub mod semantic_search_service;
pub mod recommendation_service; // Content recommendations

// v1.7.0: MCP/AI Automation Foundation
pub mod mcp_client; // Model Context Protocol client
pub mod mcp_server; // Model Context Protocol server (port 9009)
pub mod mcp_custom_tool_service; // Phase 2: Custom tool registration (HTTP webhooks, 200% isolation)
pub mod ai_providers; // AI provider abstraction (OpenAI, Anthropic, Google)
pub mod ai_key_manager; // Encrypted API key management
pub mod ai_rate_limiter; // Rate limiting & cost tracking
pub mod command_parser; // Text command parsing
pub mod command_router; // Command routing & execution
pub mod ai_command_executor; // AI command execution with real providers
pub mod ai_response_handler; // Response processing & formatting
pub mod workflow_orchestrator; // Multi-step workflow orchestration
pub mod content_templates; // Pre-built content templates
pub mod content_scheduler; // Task queue & scheduling
pub mod error_recovery; // Error handling & recovery

// v1.7.0 Phase 2: Advanced SEO/AEO/GEO
pub mod keyword_research; // Keyword research & analysis
pub mod aeo_service; // Answer Engine Optimization
pub mod geo_service; // Generative Engine Optimization
pub mod seo_meta_generator; // SEO metadata generation
pub mod schema_markup; // Schema.org structured data
pub mod content_quality; // Content quality analysis
pub mod link_builder; // Link building & analysis
pub mod seo_auditor; // Comprehensive SEO audit
pub mod competitor_analyzer; // Competitor SEO analysis
pub mod content_optimizer; // AI content optimization
pub mod performance_monitor; // Performance & Core Web Vitals
pub mod content_calendar; // Content planning & scheduling
pub mod rank_tracker; // Keyword ranking monitor
pub mod local_seo; // Local SEO & GMB optimization
pub mod voice_search; // Voice search optimization

// v1.7.0 Phase 3: E-commerce Pro
pub mod inventory_analytics; // Inventory analytics & insights
pub mod product_reviews; // Product reviews & ratings
pub mod cart_abandonment; // Cart abandonment tracking
pub mod wishlist; // Wishlist system
pub mod product_bundles; // Product bundling
pub mod conversion_analytics; // Conversion funnel analytics
pub mod product_import_export; // Bulk import/export
pub mod order_management; // Order processing
pub mod payment_processor; // Payment handling
pub mod shipping_manager; // Shipping & fulfillment
pub mod customer_support; // Support tickets

// v1.7.0 Phase 4: Performance & Benchmarking
pub mod performance_benchmark; // Performance benchmarking
pub mod load_tester; // Load testing
pub mod system_profiler; // CPU/Memory profiling
pub mod metrics_collector; // Prometheus metrics
pub mod performance_optimizer; // Query optimization
pub mod cache_manager; // Caching layer
pub mod monitoring_setup; // Monitoring configuration
pub mod documentation_generator; // Doc generation

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
pub mod metadata_automation_service; // AI metadata generation
pub mod encryption_service; // Phase 2: API key encryption
// Services below require migrations to be run first
pub mod language_service;
pub mod analytics_service_v2;
pub mod oauth_service;
pub mod hreflang_service;
pub mod audit_service;
pub mod email_service;
pub mod email_template_service;
pub mod email_verification_service; // Public API email verification
pub mod crm_service; // CRM modules below require migrations to be run first
pub mod billing_service;
pub mod billing_events;