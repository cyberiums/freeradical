pub mod config_models;
pub mod module_models;
pub mod page_models;
pub mod status_enum;  // Re-enabled with PagesStatus in schema
pub mod user_models;
pub mod tenant_models;
pub mod survey_models;
pub mod rbac;
pub mod commerce_models;
pub mod media_models;
pub mod revision_models;
pub mod field_type_enum;
pub mod category_models;
pub mod inventory_models;  // Inventory management models
pub mod crm_models; // CRM module
pub mod language_models; // Language support
pub mod translation_models; // Translation support
pub mod ai_provider_models; // AI provider configs
pub mod ai_generated_content_models; // AI generated content tracking
pub mod analytics_models; // Analytics event tracking
pub mod db_connection;  // Database abstraction layer
pub mod db_macros;      // Helper macros for database operations
pub mod theme_models;
pub mod marketplace_plugin_models;
pub mod webhook_models;
pub mod audit_models;

use actix_web::web;
use diesel::{PgConnection, query_builder::AsChangeset, r2d2::{ConnectionManager, Pool, PoolError, PooledConnection}};

use crate::services::errors_service::CustomHttpError;

use self::config_models::LocalConfig;

// Re-export database pool types (PostgreSQL only)
pub use db_connection::DatabasePool;
pub use db_connection::PooledDatabaseConnection;
pub use db_connection::ReadDatabasePool;

pub type DbPool = DatabasePool;

/// CRUD implementation.
pub trait Model<TQueryable, TMutable: AsChangeset, TPrimary, TDto = TQueryable> {
    fn create(new: &TMutable, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error>;
    fn read_one(id: TPrimary, db: &mut PooledDatabaseConnection) -> Result<TDto, diesel::result::Error>;
    fn read_all(db: &mut PooledDatabaseConnection) -> Result<Vec<TDto>, diesel::result::Error>;
    fn update(
        id: TPrimary,
        new: &TMutable,
        db: &mut PooledDatabaseConnection,
    ) -> Result<usize, diesel::result::Error>;
    fn delete(id: TPrimary, db: &mut PooledDatabaseConnection) -> Result<usize, diesel::result::Error>;
}

pub trait DTO<TColumns> {
    fn columns() -> TColumns;
}

pub trait Joinable<TLeft, TRight, TPrimary> {
    fn read_one_join_on(
        id: TPrimary,
        db: &mut PooledDatabaseConnection,
    ) -> Result<(TLeft, Vec<TRight>), diesel::result::Error>;
}

pub fn format_connection_string(conf: config_models::LocalConfig) -> String {
    // First check for DATABASE_URL environment variable (supports both MySQL and PostgreSQL)
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        return database_url;
    }
    
    // Fall back to MySQL config if available
    if let (Some(username), Some(password), Some(database), Some(url), Some(port)) = (
        conf.mysql_username,
        conf.mysql_password,
        conf.mysql_database,
        conf.mysql_url,
        conf.mysql_port,
    ) {
        if url.contains(':') {
            format!(
                "mysql://{}:{}@{}:{}/ {}",
                username,
                password,
                url.split(":").collect::<Vec<&str>>()[0],
                url.split(":").collect::<Vec<&str>>()[1],
                database
            )
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                username, password, url, port, database
            )
        }
    } else {
        panic!("Either DATABASE_URL or MySQL configuration (username, password, database, url, port) must be provided");
    }
}

pub fn establish_database_connection(conf: LocalConfig) -> Option<DatabasePool> {
    let db_url = format_connection_string(conf);
    Some(db_connection::create_pool(&db_url).expect("Failed to create pool"))
}

pub fn init_connection(db_url: &str) -> ConnectionManager<diesel::PgConnection> {
    ConnectionManager::<PgConnection>::new(db_url)
}

pub fn init_pool(db_url: &str) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = init_connection(db_url);
    Pool::builder().max_size(2).build(manager)
}

pub fn pool_handler(pool: web::Data<DatabasePool>) -> Result<PooledDatabaseConnection, CustomHttpError> {
    pool.get().or(Err(CustomHttpError::InternalServerError("Pool connection failed".to_string())))
}
