use actix_cors::Cors;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::{Connection};
use handlebars::Handlebars;
use std::sync::Mutex;
use std::time::Duration;
use envy;
use dotenv::dotenv;
// Diesel 2.x migration import
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use actix_files as fs;

mod controllers;
mod helpers;
mod services;
mod models;
mod routers;
mod schema;
mod watch;
mod graphql;

use routers::module_routers::ModuleRouter;
use routers::page_routers::PageRouter;

use models::config_models::LocalConfig;
use routers::category_routers::CategoryRouter;

use crate::routers::Router;
use crate::routers::user_routers::UserRouter;

#[macro_use]
extern crate diesel;

// Diesel 2.x migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// The main function is replaced by actix_web::main.
/// This allows main to be async and register the HttpServer.
/// All routes are defined here.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Initialize config
    let conf: LocalConfig = envy::prefixed("APP_").from_env().unwrap();

    // Run migrations - DISABLED for Docker (run manually via migrate.sh)
    // let db_url = models::format_connection_string(conf.clone());
    // let mut connection = MysqlConnection::establish(&db_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    
    // println!("Running migrations...");
    // match connection.run_pending_migrations(MIGRATIONS) {
    //     Ok(_) => println!("Ran migrations."),
    //     Err(e) => println!("Migrations error: {}", e)
    // };

    let pool = models::establish_database_connection(conf.clone()).unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();  // REMOVED - already called above at line 47

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

    let store = MemoryStore::new();

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

    let server_url = &format!(
        "{}:{}",
        &conf.bind_address,
        &conf.bind_port
    );

    let http_server = HttpServer::new(move || {
        let cors = Cors::permissive();

        let api_scope = web::scope("/v1")
            .service(UserRouter::new())
            .service(PageRouter::new())
            .service(ModuleRouter::new())
            .service(CategoryRouter::new());

        let rate_limiting = RateLimiter::new(
            MemoryStoreActor::from(store.clone()).start())
                .with_interval(Duration::from_secs(60))
                .with_max_requests(usize::from(conf.max_req));

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a -> %U | %Dms "))
            .wrap(rate_limiting)
            .service(api_scope)
            .app_data(web::Data::from(plugin_registry.clone()))
            .wrap(services::plugin_service::middleware::PluginMiddleware::new(plugin_registry.clone()))
            // GraphQL endpoint
            .app_data(graphql_schema.clone())
            .service(controllers::graphql_controller::graphql_handler)
            .service(controllers::graphql_controller::graphql_playground)
            // SEO endpoints - standardized manual routing
            .route("/sitemap.xml", web::get().to(controllers::sitemap_controller::sitemap))
            .route("/image-sitemap.xml", web::get().to(controllers::image_sitemap_controller::image_sitemap))
            // Admin/Backup endpoints
            .route("/admin/backup", web::post().to(controllers::backup_controller::create_backup))
            // Payment endpoints
                .route("/payments/create", web::post().to(controllers::payment_controller::create_payment_intent))
                .route("/payments/get", web::get().to(controllers::payment_controller::get_payment_intent))
                .route("/payments/providers", web::get().to(controllers::payment_controller::list_payment_handlers))
                // Product management routes
                .route("/products", web::get().to(controllers::product_controller::list_products))
                .route("/products/{id}", web::get().to(controllers::product_controller::get_product))
                .route("/products", web::post().to(controllers::product_controller::create_product))
                .route("/products/{id}", web::put().to(controllers::product_controller::update_product))
                .route("/products/{id}", web::delete().to(controllers::product_controller::delete_product))
                // Order management routes
                .route("/orders", web::get().to(controllers::order_controller::list_orders))
                .route("/orders/{id}", web::get().to(controllers::order_controller::get_order))
                .route("/orders", web::post().to(controllers::order_controller::create_order))
                .route("/orders/{id}/status", web::put().to(controllers::order_controller::update_order_status))
                .route("/orders/{id}/payment", web::post().to(controllers::order_controller::link_payment_to_order))
            // .service(controllers::robots_controller::robots)  // Commented - controller removed
            // Admin Dashboard API
            // .service(controllers::dashboard_controller::dashboard_summary)  // Commented - controller removed
            // .service(controllers::dashboard_controller::analytics_summary)
            // .service(controllers::dashboard_controller::seo_health)
            // .service(controllers::dashboard_controller::top_pages)
            .service(fs::Files::new("/assets", "./templates/assets").show_files_listing())
            .default_service(web::get().to(controllers::page_controllers::display_page))
            .data(pool.clone())
            .app_data(handlebars_ref.clone())
            .app_data(payment_registry.clone())
    })
    .bind(server_url)?
    .workers(2)
    .run();

    println!("🚀 Server is running 🚀");

    http_server.await
}
