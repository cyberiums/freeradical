// Database Connection Abstraction Layer
// Supports both MySQL and PostgreSQL with runtime switching

use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

/// Database type detected from DATABASE_URL
#[derive(Debug, Clone, Copy)]
pub enum DatabaseType {
    MySQL,
    Postgres,
}

/// Connection wrapper supporting both MySQL and PostgreSQL
pub enum DatabaseConnection {
    MySQL(MysqlConnection),
    Postgres(PgConnection),
}

/// Pooled connection wrapper
pub enum PooledDatabaseConnection {
    MySQL(PooledConnection<ConnectionManager<MysqlConnection>>),
    Postgres(PooledConnection<ConnectionManager<PgConnection>>),
}

/// Connection pool wrapper
#[derive(Debug, Clone)]
pub enum DatabasePool {
    MySQL(Pool<ConnectionManager<MysqlConnection>>),
    Postgres(Pool<ConnectionManager<PgConnection>>),
}

impl DatabasePool {
    /// Get a connection from the pool
    pub fn get(&self) -> Result<PooledDatabaseConnection, PoolError> {
        match self {
            DatabasePool::MySQL(pool) => {
                pool.get().map(PooledDatabaseConnection::MySQL)
            }
            DatabasePool::Postgres(pool) => {
                pool.get().map(PooledDatabaseConnection::Postgres)
            }
        }
    }
}

impl PooledDatabaseConnection {
    /// Get mutable reference to MySQL connection if available
    pub fn as_mysql_mut(&mut self) -> Option<&mut MysqlConnection> {
        match self {
            PooledDatabaseConnection::MySQL(conn) => Some(conn),
            _ => None,
        }
    }
    
    /// Get mutable reference to Postgres connection if available  
    pub fn as_postgres_mut(&mut self) -> Option<&mut PgConnection> {
        match self {
            PooledDatabaseConnection::Postgres(conn) => Some(conn),
            _ => None,
        }
    }
}

/// Execute query on either database connection type
#[macro_export]
macro_rules! with_connection {
    ($conn:expr, |$c:ident| $body:expr) => {
        match $conn {
            PooledDatabaseConnection::MySQL(ref mut $c) => $body,
            PooledDatabaseConnection::Postgres(ref mut $c) => $body,
        }
    };
}

/// Detect database type from URL format
pub fn detect_database_type(url: &str) -> DatabaseType {
    if url.starts_with("postgres://") || url.starts_with("postgresql://") {
        DatabaseType::Postgres
    } else if url.starts_with("mysql://") {
        DatabaseType::MySQL
    } else {
        panic!(
            "Unsupported DATABASE_URL format: {}. Use mysql:// or postgres://",
            url
        )
    }
}

/// Create a database pool based on URL
pub fn create_pool(url: &str) -> DatabasePool {
    match detect_database_type(url) {
        DatabaseType::MySQL => {
            let manager = ConnectionManager::<MysqlConnection>::new(url);
            let pool = Pool::builder()
                .max_size(10)
                .build(manager)
                .expect("Failed to create MySQL connection pool");
            DatabasePool::MySQL(pool)
        }
        DatabaseType::Postgres => {
            let manager = ConnectionManager::<PgConnection>::new(url);
            let pool = Pool::builder()
                .max_size(10)
                .build(manager)
                .expect("Failed to create PostgreSQL connection pool");
            DatabasePool::Postgres(pool)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_mysql() {
        let url = "mysql://user:pass@localhost/db";
        assert!(matches!(detect_database_type(url), DatabaseType::MySQL));
    }

    #[test]
    fn test_detect_postgres() {
        let url = "postgres://user:pass@localhost/db";
        assert!(matches!(
            detect_database_type(url),
            DatabaseType::Postgres
        ));
    }

    #[test]
    fn test_detect_postgresql() {
        let url = "postgresql://user:pass@localhost/db";
        assert!(matches!(
            detect_database_type(url),
            DatabaseType::Postgres
        ));
    }

    #[test]
    #[should_panic(expected = "Unsupported DATABASE_URL format")]
    fn test_detect_unsupported() {
        detect_database_type("sqlite://test.db");
    }
}
