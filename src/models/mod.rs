pub mod config_models;
pub mod module_models;
pub mod page_models;
pub mod status_enum;
pub mod user_models;
pub mod commerce_models;
pub mod media_models;
pub mod revision_models;
pub mod field_type_enum;
pub mod category_models;
pub mod db_connection;  // Database abstraction layer
pub mod db_macros;      // Helper macros for database operations

use actix_web::web;
use diesel::{MysqlConnection, query_builder::AsChangeset, r2d2::{ConnectionManager, Pool, PoolError, PooledConnection}};

use crate::services::errors_service::CustomHttpError;

use self::config_models::LocalConfig;

// Export database abstraction layer types
pub use db_connection::{DatabasePool, PooledDatabaseConnection, create_pool, detect_database_type};

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

pub fn format_connection_string(conf: LocalConfig) -> String {
    match conf.mysql_url {
        Some(mysql_url) => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                conf.mysql_username,
                conf.mysql_password,
                mysql_url,
                conf.mysql_port.unwrap(),
                conf.mysql_database
            )
        }
        None if std::env::var("MYSQL_UNIX_PORT").is_ok() => {
            format!(
                "mysql://{}:{}@/{}", conf.mysql_username,
                conf.mysql_password,
                conf.mysql_database
            )
        }
        None => {
            panic!("Must supply one of the following: [mysql_url], [sql_name | socket_dir]")
        }
    }
}

pub fn establish_database_connection(conf: LocalConfig) -> Option<DatabasePool> {
    let db_url = format_connection_string(conf);
    Some(create_pool(&db_url))
}

pub fn init_connection(db_url: &str) -> ConnectionManager<diesel::MysqlConnection> {
    ConnectionManager::<MysqlConnection>::new(db_url)
}

pub fn init_pool(db_url: &str) -> Result<Pool<ConnectionManager<MysqlConnection>>, PoolError> {
    let manager = init_connection(db_url);
    Pool::builder().max_size(2).build(manager)
}

pub fn pool_handler(pool: web::Data<DatabasePool>) -> Result<PooledDatabaseConnection, CustomHttpError> {
    pool.get().or(Err(CustomHttpError::InternalServerError))
}
