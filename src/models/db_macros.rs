// Helper macros for PostgreSQL database operations

/// Execute a query on PostgreSQL connection
#[macro_export]
macro_rules! execute_query {
    ($conn:expr, $query:expr) => {
        $query.execute($conn)
    };
}

/// Load a single result from PostgreSQL
#[macro_export]
macro_rules! load_one {
    ($conn:expr, $query:expr) => {
        $query.first($conn)
    };
}

/// Load multiple results from PostgreSQL
#[macro_export]
macro_rules! load_all {
    ($conn:expr, $query:expr) => {
        $query.load($conn)
    };
}
