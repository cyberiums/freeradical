use diesel::{PgConnection, Connection};
use std::env;

/// Establish a PostgreSQL connection for legacy services
/// TODO: Migrate remaining services to use DatabasePool
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
