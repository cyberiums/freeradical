// Test data factories for creating test fixtures
// Provides consistent test data across all tests

use freeradical::models::{PooledDatabaseConnection, user_models::*, page_models::*, module_models::*};
use diesel::RunQueryDsl;
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;

/// User test data factory
pub struct UserFactory;

impl UserFactory {
    /// Create a basic test user
    pub fn create_test_user(conn: &mut PooledDatabaseConnection, email: &str) -> Result<User, diesel::result::Error> {
        let salt = SaltString::generate(&mut rand_core::OsRng);
        let argon2 = Argon2::default();
       let password_hash = argon2
            .hash_password(b"TestPassword123!", &salt)
            .unwrap()
            .to_string();
        
        let new_user = MutUser {
            name: "Test User".to_string(),
            email: email.to_string(),
            password: password_hash,
        };
        
        match conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                use freeradical::schema::users;
                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(mysql_conn)?;
                    
                users::table
                    .order(users::id.desc())
                    .first(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                use freeradical::schema::users;
                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(pg_conn)?;
                    
                users::table
                    .order(users::id.desc())
                    .first(pg_conn)
            }
        }
    }
    
    /// Create an admin test user
    pub fn create_admin_user(conn: &mut PooledDatabaseConnection) -> Result<User, diesel::result::Error> {
        Self::create_test_user(conn, "admin@test.com")
    }
}

/// Page test data factory
pub struct PageFactory;

impl PageFactory {
    /// Create a basic test page
    pub fn create_test_page(
        conn: &mut PooledDatabaseConnection,
        author_id: i32,
        title: &str,
    ) -> Result<Page, diesel::result::Error> {
        let new_page = MutPage {
            author_id,
            category_id: None,
            title: title.to_string(),
            description: Some("Test page description".to_string()),
            content: Some("Test page content".to_string()),
            seo_title: Some(format!("{} - SEO", title)),
            seo_description: Some("Test SEO description".to_string()),
            seo_keywords: None,
            slug: Some(title.to_lowercase().replace(" ", "-")),
            canonical_url: None,
        };
        
        match conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                use freeradical::schema::pages;
                diesel::insert_into(pages::table)
                    .values(&new_page)
                    .execute(mysql_conn)?;
                    
                pages::table
                    .order(pages::id.desc())
                    .first(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                use freeradical::schema::pages;
                diesel::insert_into(pages::table)
                    .values(&new_page)
                    .execute(pg_conn)?;
                    
                pages::table
                    .order(pages::id.desc())
                    .first(pg_conn)
            }
        }
    }
}

/// Module test data factory
pub struct ModuleFactory;

impl ModuleFactory {
    /// Create a basic test module
    pub fn create_test_module(
        conn: &mut PooledDatabaseConnection,
        name: &str,
    ) -> Result<Module, diesel::result::Error> {
        let new_module = MutModule {
            name: name.to_string(),
            description: Some("Test module description".to_string()),
            category_id: None,
        };
        
        match conn {
            PooledDatabaseConnection::MySQL(ref mut mysql_conn) => {
                use freeradical::schema::modules;
                diesel::insert_into(modules::table)
                    .values(&new_module)
                    .execute(mysql_conn)?;
                    
                modules::table
                    .order(modules::id.desc())
                    .first(mysql_conn)
            }
            PooledDatabaseConnection::Postgres(ref mut pg_conn) => {
                use freeradical::schema::modules;
                diesel::insert_into(modules::table)
                    .values(&new_module)
                    .execute(pg_conn)?;
                    
                modules::table
                    .order(modules::id.desc())
                    .first(pg_conn)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db_setup::*;
    
    #[test]
    fn test_user_factory_creates_user() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        
        let user = UserFactory::create_test_user(&mut conn, "factory@test.com");
        assert!(user.is_ok());
        
        teardown_test_db(&mut conn);
    }
}
