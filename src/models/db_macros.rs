// Helper macros for database-agnostic CRUD operations
// Reduces code duplication when working with both MySQL and PostgreSQL

/// Execute a query that works on both MySQL and PostgreSQL connections
#[macro_export]
macro_rules! execute_query {
    ($conn:expr, $query:expr) => {
        match $conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                $query.execute(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                $query.execute(pg_conn)
            }
        }
    };
}

/// Load a single result from either MySQL or PostgreSQL
#[macro_export]
macro_rules! load_one {
    ($conn:expr, $query:expr) => {
        match $conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                $query.first(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                $query.first(pg_conn)
            }
        }
    };
}

/// Load multiple results from either MySQL or PostgreSQL
#[macro_export]
macro_rules! load_all {
    ($conn:expr, $query:expr) => {
        match $conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                $query.load(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                $query.load(pg_conn)
            }
        }
    };
}
