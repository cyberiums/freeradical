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

    let handlebars = Handlebars::new();

    // web::Data is Arc, so we can safely clone it and send it between our watcher and the server.
    let handlebars_ref = web::Data::new(Mutex::new(handlebars));
    let hb = handlebars_ref.clone();

    hb.lock()
        .unwrap()
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    // Registers all default handlebars functions.
    helpers::default::register_helpers(handlebars_ref.clone());

    // Registers the fs watcher that updates the templates in memory every time a template is changed.
    // This is what enables hot reload.
    std::thread::spawn(|| watch::watch(hb));

    // Initialize GraphQL Schema
    let graphql_schema = web::Data::new(graphql::create_schema());

    let store = MemoryStore::new();

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
            // GraphQL endpoint
            .app_data(graphql_schema.clone())
            .service(controllers::graphql_controller::graphql_handler)
            .service(controllers::graphql_controller::graphql_playground)
            // SEO endpoints - standardized manual routing
            .route("/sitemap.xml", web::get().to(controllers::sitemap_controller::sitemap))
            .route("/image-sitemap.xml", web::get().to(controllers::image_sitemap_controller::image_sitemap))
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
    })
    .bind(server_url)?
    .workers(2)
    .run();

    println!("🚀 Server is running 🚀");

    http_server.await
}
