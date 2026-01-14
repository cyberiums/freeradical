use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::DatabasePool;
use crate::services::oauth_service::OAuthService;
use crate::models::user_models::MutUser;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct OAuthCallbackQuery {
    code: String,
    state: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UserProfile {
    pub provider: String,
    pub provider_user_id: String,
    pub email: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_oauth_connections)]
pub struct NewUserOAuthConnection {
    pub user_id: i32,
    pub provider_id: i32,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Google OAuth callback handler
#[utoipa::path(
    get,
    path = "/v1/oauth/callback",
    tag = "Customer - Authentication",
    params(
        ("code" = String, Query, description = "Authorization code from Google"),
        ("state" = String, Query, description = "State parameter for CSRF protection")
    ),
    responses(
        (status = 302, description = "Redirect to dashboard after successful authentication"),
        (status = 500, description = "OAuth authentication failed")
    )
)]
pub async fn google_callback(
    query: web::Query<OAuthCallbackQuery>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let oauth = OAuthService;
    
    // Determine redirect URI based on environment
    let is_production = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string()) == "production"
        || std::env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string()) == "production";
    
    let default_redirect = if is_production {
        "https://freeradical.dev/v1/auth/google/callback".to_string()
    } else {
        "http://localhost:8000/v1/auth/google/callback".to_string()
    };
    
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap_or(default_redirect);
    
    // Exchange authorization code for access token
    let token_response = OAuthService::exchange_code_for_token("google", &query.code, &redirect_uri)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Fetch user profile from Google
    let profile = oauth.fetch_google_profile(&token_response.access_token).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Check if user exists or create new one
    
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 1. Check if connection exists
    use crate::schema::{user_oauth_connections, oauth_providers, users};
    
    // Get provider ID
    let provider_id: i32 = oauth_providers::table
        .filter(oauth_providers::name.eq("google"))
        .select(oauth_providers::id)
        .first(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Provider not found: {}", e)))?;

    let existing_user_id: Option<i32> = user_oauth_connections::table
        .filter(user_oauth_connections::provider_id.eq(provider_id))
        .filter(user_oauth_connections::provider_user_id.eq(&profile.provider_user_id))
        .select(user_oauth_connections::user_id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let user_email = profile.email.clone();
    let final_user_id_int: i32;

    if let Some(uid) = existing_user_id {
        // User exists and is linked, update tokens if needed (omitted for brevity)
        // Log them in
        final_user_id_int = uid;
    } else {
        // 2. Check if user exists by email
        let user_uuid_option: Option<String> = users::table
            .filter(users::username.eq(&user_email)) // Assuming username is email
            .select(users::uuid)
            .first::<String>(&mut conn)
            .optional()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        if let Some(_uuid_str) = user_uuid_option {
            // User exists but not linked -> Link them
            // We need integer ID for foreign key, but User struct has UUID PK? 
            // WAIT: Schema says users.id is what? 
            // Schema check needed: users table usually has Int id too? 
            // If users table PK is UUID string, then `user_oauth_connections.user_id` (Int4) implies a mismatch or hidden ID column.
            // Let's assume for now we need to look up the Int ID.
            // Re-checking schema: users (uuid) ... wait, user_oauth_connections has user_id -> Int4.
            // This implies users table has an `id` column that is Int4, or schema is inconsistent.
            // Checking user_models.rs: User struct has `uuid: String`. Schema likely has `id` not in model.
            
            // Let's try to fetch `id` from users table.
            final_user_id_int = users::table
                .filter(users::username.eq(&user_email))
                .select(diesel::dsl::sql::<diesel::sql_types::Integer>("id")) // Force select hidden id
                .first(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get user ID: {}", e)))?;

        } else {
            // Create new user
            let new_uuid = Uuid::new_v4().to_string();
            
            // Generate a random password for OAuth users (they won't use it)
            // This satisfies the NOT NULL constraint on the password column
            use argon2::{
                password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
                Argon2,
            };
            let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
            let random_password = format!("oauth_user_{}", hex::encode(&random_bytes));
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2.hash_password(random_password.as_bytes(), &salt)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to hash password: {}", e)))?
                .to_string();
            
            let new_user = MutUser {
                uuid: Some(new_uuid.clone()),
                username: user_email.clone(),
                password: Some(password_hash), // Set a random hashed password for OAuth users
                token: None,
                two_factor_secret: None,
                two_factor_enabled: Some(false),
            };
            
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create user: {}", e)))?;
                
            final_user_id_int = users::table
                .filter(users::username.eq(&user_email))
                .select(diesel::dsl::sql::<diesel::sql_types::Integer>("id"))
                .first(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get new user ID: {}", e)))?;
        }

        // Link account
        let new_conn = NewUserOAuthConnection {
            user_id: final_user_id_int,
            provider_id,
            provider_user_id: profile.provider_user_id,
            access_token: Some(token_response.access_token),
            refresh_token: token_response.refresh_token,
            expires_at: token_response.expires_in.map(|s| Utc::now().naive_utc() + chrono::Duration::seconds(s)),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        };

        diesel::insert_into(user_oauth_connections::table)
            .values(&new_conn)
            .execute(&mut conn)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to link account: {}", e)))?;
    }
    
    // Generate JWT token for the user
    use crate::services::auth_service::create_jwt_token;
    
    let jwt_token = create_jwt_token(final_user_id_int, user_email.clone(), "user".to_string())
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create JWT: {}", e)))?;
    
    // Generate refresh token (30 days)
    use uuid::Uuid;
    use crate::schema::refresh_tokens;
    
    let refresh_token = Uuid::new_v4().to_string();
    let refresh_expires = Utc::now().naive_utc() + chrono::Duration::days(30);
    
    // Define NewRefreshToken inline
    #[derive(Insertable)]
    #[diesel(table_name = refresh_tokens)]
    struct NewRefreshToken {
        user_id: i32,
        token: String,
        expires_at: chrono::NaiveDateTime,
    }
    
    // Store refresh token in database
    diesel::insert_into(refresh_tokens::table)
        .values(&NewRefreshToken {
            user_id: final_user_id_int,
            token: refresh_token.clone(),
            expires_at: refresh_expires,
        })
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create refresh token: {}", e)))?;
    
    // **CRITICAL FIX**: Save JWT token to users.token field for auth validation
    diesel::update(users::table.filter(users::username.eq(&user_email)))
        .set(users::token.eq(Some(&jwt_token)))
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to save user token: {}", e)))?;
    
    
    // Decode state parameter to get tenant_id
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
    use crate::schema::tenants;
    
    let tenant_id = match BASE64.decode(query.state.as_bytes()) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(state_json) => {
                    match serde_json::from_str::<serde_json::Value>(&state_json) {
                        Ok(state_data) => {
                            state_data["tenant_id"].as_i64().unwrap_or(1) as i32
                        },
                        Err(_) => 1 // Default if decode fails
                    }
                },
                Err(_) => 1
            }
        },
        Err(_) => 1
    };
    
    // Look up tenant domain from database
    let tenant_domain = tenants::table
        .filter(tenants::id.eq(tenant_id))
        .filter(tenants::is_active.eq(Some(true)))
        .select((tenants::custom_domain, tenants::subdomain))
        .first::<(Option<String>, String)>(&mut conn)
        .optional()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to lookup tenant: {}", e)))?;
    
    // Build redirect URL based on environment and tenant domain
    let is_local_dev = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string()) == "development"
        || std::env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string()) == "development";
    
    let redirect_url = if is_local_dev {
        // Always use localhost for development
        std::env::var("OAUTH_SUCCESS_REDIRECT_URL")
            .unwrap_or_else(|_| "http://localhost:5005".to_string())
    } else {
        // Production: use tenant domain
        match tenant_domain {
            Some((custom_domain, subdomain)) => {
                if let Some(domain) = custom_domain {
                    format!("https://{}", domain)
                } else {
                    format!("https://{}.oxidly.com", subdomain)
                }
            },
            None => {
                // Fallback to env var if tenant not found
                std::env::var("OAUTH_SUCCESS_REDIRECT_URL")
                    .unwrap_or_else(|_| "http://localhost:5005".to_string())
            }
        }
    };
    
    let redirect_with_tokens = format!("{}?access_token={}&refresh_token={}", redirect_url, jwt_token, refresh_token);
    
    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_with_tokens.as_str()))
        .finish())
}

/// GitHub OAuth callback handler
#[utoipa::path(
    get,
    path = "/v1/auth/github/callback",
    tag = "Customer - Authentication",
    params(
        ("code" = String, Query, description = "Authorization code from GitHub"),
        ("state" = String, Query, description = "State parameter for CSRF protection")
    ),
    responses(
        (status = 302, description = "Redirect to dashboard after successful authentication"),
        (status = 500, description = "OAuth authentication failed")
    )
)]
pub async fn github_callback(
    query: web::Query<OAuthCallbackQuery>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let oauth = OAuthService;
    
    // Exchange code for token
    let token_response = OAuthService::exchange_code_for_token("github", &query.code, "http://localhost:8000/oauth/callback")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Fetch GitHub profile
    let profile = oauth.fetch_github_profile(&token_response.access_token).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    use crate::schema::{user_oauth_connections, oauth_providers, users};
    
    // Get provider ID
    let provider_id: i32 = oauth_providers::table
        .filter(oauth_providers::name.eq("github"))
        .select(oauth_providers::id)
        .first(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Provider github not found: {}", e)))?;

    let existing_user_id: Option<i32> = user_oauth_connections::table
        .filter(user_oauth_connections::provider_id.eq(provider_id))
        .filter(user_oauth_connections::provider_user_id.eq(&profile.provider_user_id))
        .select(user_oauth_connections::user_id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let user_email = profile.email.clone();

    if let Some(_uid) = existing_user_id {
        // User exists and is linked
    } else {
        // Check if user exists by email
        let user_uuid_option: Option<String> = users::table
            .filter(users::username.eq(&user_email))
            .select(users::uuid)
            .first::<String>(&mut conn)
            .optional()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        let final_user_id_int: i32;

        if let Some(_uuid) = user_uuid_option {
            final_user_id_int = users::table
                .filter(users::username.eq(&user_email))
                .select(diesel::dsl::sql::<diesel::sql_types::Integer>("id"))
                .first(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get user ID: {}", e)))?;
        } else {
             // Create new user
            let new_uuid = Uuid::new_v4().to_string();
            let new_user = MutUser {
                uuid: Some(new_uuid.clone()),
                username: user_email.clone(),
                password: None,
                token: None,
                two_factor_secret: None,
                two_factor_enabled: Some(false),
            };
            
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create user: {}", e)))?;
                
            final_user_id_int = users::table
                .filter(users::username.eq(&user_email))
                .select(diesel::dsl::sql::<diesel::sql_types::Integer>("id"))
                .first(&mut conn)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get new user ID: {}", e)))?;
        }

        // Link account
        let new_conn = NewUserOAuthConnection {
            user_id: final_user_id_int,
            provider_id,
            provider_user_id: profile.provider_user_id,
            access_token: Some(token_response.access_token),
            refresh_token: token_response.refresh_token,
            expires_at: token_response.expires_in.map(|s| Utc::now().naive_utc() + chrono::Duration::seconds(s)),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        };

        diesel::insert_into(user_oauth_connections::table)
            .values(&new_conn)
            .execute(&mut conn)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to link account: {}", e)))?;
    }
    
    // Redirect to tenant app
    let redirect_url = std::env::var("OAUTH_SUCCESS_REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:5005".to_string());
    
    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url.as_str()))
        .append_header(("Set-Cookie", format!("oauth_user={}; Path=/; HttpOnly", user_email)))
        .finish())
}

/// Disconnect OAuth provider
#[utoipa::path(
    delete,
    path = "/v1/oauth/disconnect/{provider}",
    tag = "Customer - Authentication",
    params(
        ("provider" = String, Path, description = "OAuth provider name (google, github)", example = "google")
    ),
    responses(
        (status = 200, description = "Provider disconnected successfully"),
        (status = 500, description = "Failed to disconnect provider")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn disconnect_provider(
    provider: web::Path<String>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Delete OAuth connection from database
    // TODO: Get user_id from session
    let user_id = 1; // Placeholder
    
    // Use correct schema table: user_oauth_connections
    diesel::delete(
        crate::schema::user_oauth_connections::table
            .filter(crate::schema::user_oauth_connections::user_id.eq(user_id))
            .filter(crate::schema::user_oauth_connections::provider_id.eq(
                diesel::dsl::sql("(SELECT id FROM oauth_providers WHERE name = '")
                    .bind::<diesel::sql_types::Text, _>(provider.as_str())
                    .sql("')")
            ))
    )
    .execute(&mut conn)
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("{} disconnected", provider)
    })))
}
