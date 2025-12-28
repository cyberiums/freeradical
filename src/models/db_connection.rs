// Database Connection - PostgreSQL Only
// Simplified for performance and AI feature support (pgvector)

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

/// Database type - PostgreSQL only
#[derive(Debug, Clone, Copy)]
pub enum DatabaseType {
    Postgres,
}

/// Connection pool wrapper
pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

/// Read Replica Connection Poll Wrapper (Newtype to distinguish in Actix)
#[derive(Clone)]
pub struct ReadDatabasePool(pub DatabasePool);

/// Pooled connection
pub type PooledDatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Detect database type from DATABASE_URL (always Postgres)
pub fn detect_database_type(database_url: &str) -> DatabaseType {
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        DatabaseType::Postgres
    } else {
        panic!("Only PostgreSQL is supported. DATABASE_URL must start with postgres:// or postgresql://");
    }
}

/// Create a connection pool
pub fn create_pool(database_url: &str) -> Result<DatabasePool, PoolError> {
    let _db_type = detect_database_type(database_url);
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
}

/// Get database type string for logging
pub fn get_database_type_string(pool: &DatabasePool) -> &'static str {
    "PostgreSQL"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_postgres() {
        assert!(matches!(
            detect_database_type("postgresql://localhost/test"),
            DatabaseType::Postgres
        ));
    }

    #[test]
    #[should_panic(expected = "Only PostgreSQL is supported")]
    fn test_reject_mysql() {
        detect_database_type("mysql://localhost/test");
    }
}
