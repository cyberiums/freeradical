// Database connection service
// Provides centralized database connection handling

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::env;

/// Establishes a connection to the MySQL database
/// Uses DATABASE_URL environment variable
pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            // Fallback: construct from individual env vars
            let user = env::var("APP_MYSQL_USERNAME").unwrap_or_else(|_| "rustcms".to_string());
            let pass = env::var("APP_MYSQL_PASSWORD").unwrap_or_else(|_| "password".to_string());
            let host = env::var("APP_MYSQL_URL").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("APP_MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
            let db = env::var("APP_MYSQL_DATABASE").unwrap_or_else(|_| "rustcms".to_string());
            
            format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)
        });
    
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
