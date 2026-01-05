use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::Connection;
use envy;
use dotenv::dotenv;
// Diesel 2.x migration import
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// OpenAPI/Swagger imports
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};

use actix_files as fs;

mod controllers;
mod helpers;
mod services;
mod models;
mod routers;
mod schema;
mod sql_types;
mod watch;
mod graphql;
mod api_docs;
mod middleware;

use routers::module_routers::ModuleRouter;
use routers::page_routers::PageRouter;
use services::storage_service::StorageBackend;

use models::config_models::LocalConfig;
use routers::category_routers::CategoryRouter;

use crate::routers::Router;
use crate::routers::user_routers::UserRouter;

#[macro_use]
extern crate diesel;

// Diesel 2.x migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_postgres");

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Initialize config
    let conf: LocalConfig = envy::prefixed("APP_").from_env().unwrap();

    // Run migrations
    let db_url_for_migration = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&db_url_for_migration)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url_for_migration));
    
    println!("Running migrations...");
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations complete"),
        Err(e) => println!("Migrations error: {}", e)
    };

    // Create database URL from config
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = match models::db_connection::create_pool(&db_url) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create database pool: {:?}", e);
            std::process::exit(1);
        }
    };

    // Initialize Read Replica Pool (or fallback to primary)
    let read_pool = match conf.database_url_read.as_ref() {
        Some(url) => {
            log::info!("Initializing Read Replica Pool...");
            match models::db_connection::create_pool(url) {
                Ok(p) => models::db_connection::ReadDatabasePool(p),
                Err(e) => {
                     log::error!("Failed to connect to Read Replica: {}, falling back to primary", e);
                     models::db_connection::ReadDatabasePool(pool.clone())
                }
            }
        },
        None => models::db_connection::ReadDatabasePool(pool.clone())
    };

    // Initialize Storage Backend
    let storage_backend = StorageBackend::from_config(&conf).await;

    // Initialize Cache Service
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    // Default TTL: 300 seconds (5 mins)
    let cache_service = services::cache_service_v2::CacheServiceV2::new(
        &redis_url, 
        conf.redis_cluster_nodes.clone(),
        300
    ).await.expect("Failed to initialize Cache Service");
    let cache_service = web::Data::new(cache_service);

    // Initialize Template Service (Supports Handlebars + Liquid)
    let template_service = services::template_service::TemplateService::new();
    
    // Register templates
    template_service.load_templates("./templates").unwrap();
    
    // Legacy support: Create web::Data from the inner Arc that TemplateService holds.
    // This allows existing controllers to continue accessing "Data<Mutex<Handlebars>>"
    let handlebars_ref = web::Data::from(template_service.get_handlebars());
    
    // Registers all default handlebars functions.
    helpers::default::register_helpers(handlebars_ref.clone());

    // Registers the fs watcher
    // We pass the legacy ref to the watcher for now.
    // Ideally, watcher should trigger template_service.reload()
    let hb_for_watch = handlebars_ref.clone();
    std::thread::spawn(|| watch::watch(hb_for_watch));

    // Initialize GraphQL Schema
    let graphql_schema = web::Data::new(graphql::create_schema());

    // Configure rate limiting with actix-governor
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)  // 1 request per second = 60 per minute
        .burst_size(conf.max_req.into())
        .finish()
        .unwrap();

    // Auto-Run Migrations on Startup
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_postgres");
    
    let mut conn = pool.get().expect("Failed to get DB connection for migrations");
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => log::info!("Database migrations executed successfully."),
        Err(e) => log::error!("Failed to run database migrations: {}", e),
    };

    // Initialize Plugin Registry
    let plugin_registry = std::sync::Arc::new(services::plugin_service::PluginRegistry::new());
    plugin_registry.load_all().await;

    // Initialize Payment Handler Registry
    let mut payment_registry = services::payment_service::PaymentHandlerRegistry::new();
    
    // Register Stripe handler if API key is present
    if let Ok(stripe_key) = std::env::var("STRIPE_SECRET_KEY") {
        payment_registry.register(Box::new(
            services::payment_service::stripe::StripePaymentHandler::new(stripe_key)
        ));
    }
    
    // Register PayPal handler if credentials are present
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("PAYPAL_CLIENT_ID"),
        std::env::var("PAYPAL_CLIENT_SECRET")
    ) {
        let sandbox = std::env::var("PAYPAL_SANDBOX").unwrap_or_else(|_| "true".to_string()) == "true";
        payment_registry.register(Box::new(
            services::payment_service::paypal::PayPalPaymentHandler::new(client_id, client_secret, sandbox)
        ));
    }
    
    // Register Square handler if access token is present
    if let Ok(access_token) = std::env::var("SQUARE_ACCESS_TOKEN") {
        let sandbox = std::env::var("SQUARE_SANDBOX").unwrap_or_else(|_| "true".to_string()) == "true";
        payment_registry.register(Box::new(
            services::payment_service::square::SquarePaymentHandler::new(access_token, sandbox)
        ));
    }
    
    let payment_registry = web::Data::new(payment_registry);

    // Initialize Email Service
    let email_template_service = services::email_template_service::EmailTemplateService::new();
    let email_service = services::email_service::EmailService::new(email_template_service).await;
    let email_service = web::Data::new(email_service);

    let server_url = &format!(
        "{}:{}",
        &conf.bind_address,
        &conf.bind_port
    );

    // Clone pool for MCP Server before HttpServer closure captures it
    let pool_for_mcp = pool.clone();
    let pool_for_cleanup = pool.clone();

    let email_service_for_server = email_service.clone();
    let http_server = HttpServer::new(move || {
        let cors = Cors::permissive();

        let api_scope = web::scope("/v1")
            .service(UserRouter::new())
            .service(PageRouter::new())
            .service(ModuleRouter::new())
            .service(CategoryRouter::new())
            // SEO endpoints - standardized manual routing
            .route("/sitemap.xml", web::get().to(controllers::sitemap_controller::sitemap))
            .route("/image-sitemap.xml", web::get().to(controllers::image_sitemap_controller::image_sitemap))
            // Payment endpoints
            .route("/payments/create", web::post().to(controllers::payment_controller::create_payment_intent))
            .route("/payments/get", web::get().to(controllers::payment_controller::get_payment_intent))
            .route("/payments/providers", web::get().to(controllers::payment_controller::list_payment_handlers))
            // Product management routes
            .route("/products", web::get().to(controllers::product_controller::list_products)) // Commerce disabled
            .route("/products/{id}", web::get().to(controllers::product_controller::get_product)) // Commerce disabled
            .route("/products", web::post().to(controllers::product_controller::create_product)) // Commerce disabled
            .route("/products/{id}", web::put().to(controllers::product_controller::update_product)) // Commerce disabled
            .route("/products/{id}", web::delete().to(controllers::product_controller::delete_product)) // Commerce disabled
            // Order management routes
            .route("/orders", web::get().to(controllers::order_controller::list_orders)) // Commerce disabled
            .route("/orders/{id}", web::get().to(controllers::order_controller::get_order)) // Commerce disabled
            .route("/orders", web::post().to(controllers::order_controller::create_order)) // Commerce disabled
            .route("/orders/{id}/status", web::put().to(controllers::order_controller::update_order_status)) // Commerce disabled
            .route("/orders/{id}/payment", web::post().to(controllers::order_controller::link_payment_to_order)) // Commerce disabled
            // Inventory management routes
            .route("/products/{id}/variants", web::get().to(services::inventory_service::get_product_variants))
            .route("/variants", web::post().to(services::inventory_service::create_variant))
            .route("/variants/{id}/stock", web::put().to(services::inventory_service::update_variant_stock))
            // .route("/products/{id}/inventory/audit", web::get().to(services::inventory_service::get_inventory_audit_log))
            .route("/variants/{id}", web::delete().to(services::inventory_service::delete_variant))
            // Tenant routes
            .route("/api/tenants", web::post().to(controllers::tenant_controller::create_tenant))
            .route("/api/tenants", web::get().to(controllers::tenant_controller::list_my_tenants))
            .route("/api/tenants/{id}/members", web::post().to(controllers::tenant_controller::invite_member))
            .route("/api/tenants/{id}", web::get().to(controllers::tenant_controller::get_tenant_details))
            // SSO Routes (SAML + OIDC)
            .route("/sso/metadata", web::get().to(controllers::tenant_sso_controller::metadata))
            .route("/sso/login/{tenant_id}", web::get().to(controllers::tenant_sso_controller::login))
            .route("/sso/saml/acs", web::post().to(controllers::tenant_sso_controller::saml_acs))
            .route("/sso/oidc/callback", web::get().to(controllers::tenant_sso_controller::oidc_callback))
            .route("/api/tenants/{id}/sso", web::post().to(controllers::tenant_sso_controller::update_config))
            .route("/api/tenants/{id}/settings", web::put().to(controllers::tenant_controller::update_settings))
            .route("/api/tenants/{id}/audit-logs", web::get().to(controllers::tenant_controller::list_audit_logs))
            .route("/api/tenants/{id}/webhooks", web::get().to(controllers::webhook_controller::list_webhooks))
            .route("/api/tenants/{id}/webhooks", web::post().to(controllers::webhook_controller::create_webhook))
            .route("/api/tenants/{id}/webhooks/{hook_id}", web::delete().to(controllers::webhook_controller::delete_webhook))
            // Survey Routes
            .route("/surveys", web::get().to(controllers::survey_controller::list_surveys))
            .route("/surveys", web::post().to(controllers::survey_controller::create_survey))
            .route("/surveys/{id}", web::get().to(controllers::survey_controller::get_survey))
            .route("/surveys/{id}/questions", web::post().to(controllers::survey_controller::add_question))
            .route("/surveys/{id}/responses", web::post().to(controllers::survey_controller::submit_response))
            .route("/surveys/{id}/results", web::get().to(controllers::survey_controller::get_results))
            // Public CRM Routes (No Auth Required - Email Verification)
            .route("/public/crm/customers", web::post().to(controllers::crm_controller_public::create_customer_public))
            .route("/public/crm/verify/{token}", web::get().to(controllers::crm_controller_public::verify_customer_email))
            // Site routes (Oxidly alias)
            .route("/sites", web::get().to(controllers::site_controller::list_sites))
            .route("/sites", web::post().to(controllers::site_controller::create_site))
            .route("/sites/validate-cname", web::post().to(controllers::site_controller::validate_cname))
            .route("/sites/{id}", web::get().to(controllers::site_controller::get_site))
            // Theme routes
            .route("/themes", web::get().to(controllers::theme_controller::list_themes))
            .route("/themes/upload", web::post().to(controllers::theme_controller::upload_theme))
            .route("/themes/{id}/activate", web::post().to(controllers::theme_controller::activate_theme))
            // Plugin Marketplace routes
            .route("/plugins", web::get().to(controllers::marketplace_plugin_controller::list_plugins))
            .route("/plugins/submit", web::post().to(controllers::marketplace_plugin_controller::submit_plugin))
            .route("/plugins/install", web::post().to(controllers::marketplace_plugin_controller::install_plugin))
            .route("/plugins/approve", web::post().to(controllers::marketplace_plugin_controller::approve_plugin))
            // User Management Extensions
            .route("/users/invite", web::post().to(controllers::user_api_extensions::invite_user))
            .route("/users/export", web::get().to(controllers::user_api_extensions::export_users))
            // Email Template Testing
            .route("/email-templates/test", web::post().to(controllers::user_api_extensions::test_email_template))
            // CRM API Routes
            .route("/api/crm/customers", web::get().to(controllers::crm_controller::list_customers))
            .route("/api/crm/customers/{id}", web::get().to(controllers::crm_controller::get_customer_profile))
            .route("/api/crm/customers", web::post().to(controllers::crm_controller::create_customer))
            .route("/api/crm/customers/{id}", web::put().to(controllers::crm_controller::update_customer))
            .route("/api/crm/customers/{id}", web::delete().to(controllers::crm_controller::delete_customer))
            .route("/api/crm/customers/{id}/timeline", web::get().to(controllers::crm_controller::get_customer_timeline))
            .route("/api/crm/customers/{id}/notes", web::post().to(controllers::crm_controller::add_customer_note))
            .route("/api/crm/customers/{id}/notes", web::get().to(controllers::crm_controller::get_customer_notes))
            .route("/api/crm/interactions", web::post().to(controllers::crm_controller::create_interaction))
            .route("/api/crm/interactions/{id}", web::get().to(controllers::crm_controller::get_interaction))
            .route("/api/crm/segments", web::get().to(controllers::crm_controller::list_segments))
            .route("/api/crm/segments", web::post().to(controllers::crm_controller::create_segment))
            .route("/api/crm/segments/{id}/members", web::get().to(controllers::crm_controller::get_segment_members))
            .route("/api/crm/campaigns", web::post().to(controllers::crm_controller::create_campaign))
            .route("/api/crm/campaigns", web::get().to(controllers::crm_controller::list_campaigns))
            .route("/api/crm/tasks", web::post().to(controllers::crm_controller::create_task))
            .route("/api/crm/tasks", web::get().to(controllers::crm_controller::list_tasks))
            .route("/api/crm/tasks/{id}", web::put().to(controllers::crm_controller::update_task))
            // Verification Settings (Admin only - works for any verification type)
            .route("/api/verification/settings", web::get().to(controllers::verification_settings_controller::get_verification_settings))
            .route("/api/verification/settings/{verification_type}", web::put().to(controllers::verification_settings_controller::update_verification_settings))
            .route("/api/verification/settings/{verification_type}", web::delete().to(controllers::verification_settings_controller::delete_verification_settings))
            // AI Provider management routes (admin only)
            .route("/admin/ai/providers", web::get().to(services::ai_provider_service::list_providers))
            .route("/admin/ai/providers/{id}", web::get().to(services::ai_provider_service::get_provider))
            .route("/admin/ai/providers", web::post().to(services::ai_provider_service::create_provider))
            .route("/admin/ai/providers/{id}", web::put().to(services::ai_provider_service::update_provider))
            .route("/admin/ai/providers/{id}", web::delete().to(services::ai_provider_service::delete_provider))
            .route("/admin/ai/providers/test", web::post().to(services::ai_provider_service::test_provider))
            // AI Content Generation routes
            .route("/v1/ai/generate/content", web::post().to(services::ai_content_service::generate_content))
            .route("/v1/ai/generate/image", web::post().to(services::ai_content_service::generate_image))
            // AI Metadata Automation routes
            .route("/ai/metadata/keywords", web::post().to(services::metadata_automation_service::extract_keywords))
            .route("/ai/metadata/tags", web::post().to(services::metadata_automation_service::generate_tags))
            .route("/ai/metadata/categories", web::post().to(services::metadata_automation_service::suggest_categories))
            .route("/ai/metadata/alt-text", web::post().to(services::metadata_automation_service::generate_alt_text))
            .route("/ai/metadata/all", web::post().to(services::metadata_automation_service::generate_all_metadata))
            .route("/search/embedding", web::post().to(services::semantic_search_service::create_embedding))
            .route("/search/semantic", web::post().to(services::semantic_search_service::semantic_search))
            .route("/v1/ai/analyze/sentiment", web::post().to(services::ai_content_service::analyze_sentiment))
            .route("/v1/ai/analyze/fraud", web::post().to(services::ai_content_service::detect_fraud))
            .route("/v1/ai/analyze/pricing", web::post().to(services::ai_content_service::analyze_pricing))
            .route("/v1/ai/forecast/supply", web::post().to(services::ai_content_service::forecast_supply))
            .route("/ai/marketing/generate", web::post().to(services::ai_content_service::generate_marketing_campaign))
            .route("/ai/marketing/optimize", web::post().to(services::ai_content_service::optimize_ad_spend))
            .route("/ai/chat/concierge", web::post().to(services::ai_content_service::chat_concierge))
            .route("/ai/architect/generate", web::post().to(services::ai_content_service::architect_theme))
            .route("/ai/vto/generate", web::post().to(services::ai_content_service::virtual_try_on))
            .route("/ai/crm/health", web::post().to(services::ai_content_service::calculate_customer_health))
            .route("/ai/crm/returns/analyze", web::post().to(services::ai_content_service::analyze_return_request))
            .route("/ai/crm/outreach/draft", web::post().to(services::ai_content_service::generate_outreach_message))
            .route("/v1/recommendations/related", web::post().to(services::recommendation_service::get_related_content))
            .route("/v1/recommendations/trending", web::get().to(services::recommendation_service::get_trending))
            // Admin Dashboard API
            .service(controllers::dashboard_controller::dashboard_summary)
            .service(controllers::dashboard_controller::analytics_summary)
            .service(controllers::dashboard_controller::seo_health)
            .service(controllers::dashboard_controller::top_pages)
            // SEO Tools
            .service(controllers::seo_controller::audit_url)
            // Backups
            .service(controllers::backup_controller::list_backups)
            .service(controllers::backup_controller::create_backup)
            // Media routes
            .route("/api/media", web::get().to(controllers::media_controller::list_media))
            .route("/api/media/{uuid}", web::get().to(controllers::media_controller::get_media))
            .route("/api/media/{uuid}", web::delete().to(controllers::media_controller::delete_media))
            .route("/api/media/upload", web::post().to(controllers::media_controller::upload_media))
            // Billing & Invoicing
            .configure(controllers::billing_controller::init_routes)
            // Custom MCP Tools & Marketplace (Phase 2)
            .configure(controllers::mcp_custom_tool_controller::init_routes)
            // OAuth Redirects
            .route("/auth/google", web::get().to(controllers::oauth_controller::google_login))
            .route("/auth/google/callback", web::get().to(controllers::oauth_callback_controller::google_callback))
            // Refresh Token endpoints
            .route("/auth/refresh", web::post().to(controllers::refresh_controller::refresh_token))
            .route("/auth/revoke", web::post().to(controllers::refresh_controller::revoke_token));


        App::new()
            // Middleware
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(actix_web::middleware::Compress::default()) // Compression (Gzip/Brotli)
            .wrap(middleware::security_headers::SecurityHeaders) // Security Headers
            .wrap(Governor::new(&governor_conf))
            .wrap(middleware::AuthMiddleware::new())  // JWT authentication
            // App data
            .service(api_scope)
            // OpenAPI/Swagger Documentation
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api_docs::ApiDoc::openapi())
            )
            .service(
                Redoc::with_url("/redoc", api_docs::ApiDoc::openapi())
            )
            .route("/api-docs", web::get().to(|| async {
                actix_web::HttpResponse::Ok().content_type("text/html").body(r#"
<!DOCTYPE html>
<html>
<head>
    <title>FreeRadical API Docs</title>
    <style>
        body { font-family: sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }
        .card { background: #f7f7f7; padding: 20px; margin: 15px 0; border-radius: 8px; }
        a { color: #4299e1; text-decoration: none; font-weight: 600; }
    </style>
</head>
<body>
    <h1>🚀 FreeRadical API Documentation</h1>
    <div class="card">
        <h2>Swagger UI</h2>
        <a href="/swagger-ui/">Open →</a>
    </div>
    <div class="card">
        <h2>ReDoc</h2>
        <a href="/redoc">Open →</a>
    </div>
</body>
</html>
                "#)
            }))
            .app_data(web::Data::from(plugin_registry.clone()))
            .wrap(services::plugin_service::middleware::PluginMiddleware::new(plugin_registry.clone()))
            // GraphQL endpoint
            .app_data(graphql_schema.clone())
            .service(controllers::graphql_controller::graphql_handler)
            .service(controllers::graphql_controller::graphql_playground)

            .route("/", web::get().to(|| async {
                actix_files::NamedFile::open_async("./static/index.html").await
            }))
            .route("/manifest.json", web::get().to(|| async {
                actix_files::NamedFile::open_async("./static/icons/manifest.json").await
            }))
            .route("/admin", web::get().to(|| async {
                actix_files::NamedFile::open_async("./static/admin-portal.html").await
            }))
            .service(fs::Files::new("/assets", "./templates/assets").show_files_listing())
            .service(fs::Files::new("/static", "./static"))
            .default_service(web::get().to(controllers::page_controllers::display_page))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(read_pool.clone())) // Read Replica
            .app_data(web::Data::new(storage_backend.clone())) // Storage Backend
            .app_data(cache_service.clone()) // Cache Service
            .app_data(handlebars_ref.clone())
            .app_data(payment_registry.clone())
            .app_data(email_service_for_server.clone())
    })
    .bind(server_url)?
    .workers(2)
    .run();

    // Start MCP Server on port 9009 (parallel to main REST API on port 8000)
    actix_web::rt::spawn(async move {
        log::info!("🔌 Starting FreeRadical MCP Server on port 9009...");
        match services::mcp_server::start_mcp_server(pool_for_mcp).await {
            Ok(_) => log::info!("MCP Server stopped"),
            Err(e) => log::error!("❌ MCP Server failed to start: {}", e),
        }
    });

    // Start Scheduled Tasks
    let email_service_for_sched = email_service.clone();
    actix_web::rt::spawn(async move {
        let _ = services::scheduler_service::start_scheduler(email_service_for_sched).await;
    });
    
    // Start Email Verification Cleanup Job (runs every hour)
    actix_web::rt::spawn(async move {
        use services::email_verification_service::EmailVerificationService;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; // 1 hour
            match EmailVerificationService::cleanup_expired(&pool_for_cleanup).await {
                Ok(deleted) => log::info!("✅ Cleaned up {} expired email verifications", deleted),
                Err(e) => log::error!("❌ Verification cleanup failed: {}", e),
            }
        }
    });

    println!("🚀 Server is running 🚀");

    http_server.await
}
// force rebuild
