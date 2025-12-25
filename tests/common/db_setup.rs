// Test database setup and teardown utilities
// Supports both MySQL and PostgreSQL based on TEST_DATABASE_URL

use freeradical::models::{DatabasePool, PooledDatabaseConnection, create_pool};
use diesel::RunQueryDsl;
use std::env;

/// Setup test database connection
/// Uses TEST_DATABASE_URL env var, defaults to MySQL
pub fn setup_test_db() -> DatabasePool {
    let db_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| {
            "mysql://root:password@localhost:5506/freeradical_test".to_string()
        });
    
    create_pool(&db_url)
}

/// Clear all tables for clean test state
pub fn clear_all_tables(conn: &mut PooledDatabaseConnection) {
    use freeradical::schema::*;
    
    // Use pattern matching for database-specific connection
    match conn {
        PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
            // Disable foreign key checks temporarily
            diesel::sql_query("SET FOREIGN_KEY_CHECKS=0")
                .execute(mysql_conn)
                .ok();
            
            // Clear all tables in reverse dependency order
            diesel::delete(pages::table).execute(mysql_conn).ok();
            diesel::delete(modules::table).execute(mysql_conn).ok();
            diesel::delete(categories::table).execute(mysql_conn).ok();
            diesel::delete(users::table).execute(mysql_conn).ok();
            diesel::delete(media::table).execute(mysql_conn).ok();
            diesel::delete(products::table).execute(mysql_conn).ok();
            
            // Re-enable foreign key checks
            diesel::sql_query("SET FOREIGN_KEY_CHECKS=1")
                .execute(mysql_conn)
                .ok();
        }
        PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
            // PostgreSQL version - disable triggers temporarily
            diesel::sql_query("SET session_replication_role = 'replica'")
                .execute(pg_conn)
                .ok();
            
            diesel::delete(pages::table).execute(pg_conn).ok();
            diesel::delete(modules::table).execute(pg_conn).ok();
            diesel::delete(categories::table).execute(pg_conn).ok();
            diesel::delete(users::table).execute(pg_conn).ok();
            diesel::delete(media::table).execute(pg_conn).ok();
            diesel::delete(products::table).execute(pg_conn).ok();
            
            diesel::sql_query("SET session_replication_role = 'origin'")
                .execute(pg_conn)
                .ok();
        }
    }
}

/// Teardown test database - clear all data
pub fn teardown_test_db(conn: &mut PooledDatabaseConnection) {
    clear_all_tables(conn);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_setup_creates_pool() {
        let pool = setup_test_db();
        assert!(pool.get().is_ok());
    }
}
