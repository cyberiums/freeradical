use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::models::MySQLPool;
use crate::services::oauth_service::OAuthService;

#[derive(Deserialize)]
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

#[derive(Serialize)]
struct UserProfile {
    provider: String,
    provider_user_id: String,
    email: String,
    name: String,
}

/// Google OAuth callback handler
pub async fn google_callback(
    query: web::Query<OAuthCallbackQuery>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, Error> {
    let oauth = OAuthService::new();
    
    // Exchange authorization code for access token
    let token_response = oauth.exchange_google_code(&query.code).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Fetch user profile from Google
    let profile = oauth.fetch_google_profile(&token_response.access_token).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Store OAuth connection in database
    let conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    oauth.store_connection(
        &conn,
        &profile.provider_user_id,
        "google",
        &token_response.access_token,
        token_response.refresh_token.as_deref(),
        token_response.expires_in,
    ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Create session and redirect to dashboard
    Ok(HttpResponse::Found()
        .append_header(("Location", "/admin/dashboard"))
        .append_header(("Set-Cookie", format!("oauth_user={}", profile.email)))
        .finish())
}

/// GitHub OAuth callback handler
pub async fn github_callback(
    query: web::Query<OAuthCallbackQuery>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, Error> {
    let oauth = OAuthService::new();
    
    // Exchange code for token
    let token_response = oauth.exchange_github_code(&query.code).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Fetch GitHub profile
    let profile = oauth.fetch_github_profile(&token_response.access_token).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Store connection
    let conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    oauth.store_connection(
        &conn,
        &profile.provider_user_id,
        "github",
        &token_response.access_token,
        token_response.refresh_token.as_deref(),
        token_response.expires_in,
    ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Found()
        .append_header(("Location", "/admin/dashboard"))
        .append_header(("Set-Cookie", format!("oauth_user={}", profile.email)))
        .finish())
}

/// Disconnect OAuth provider
pub async fn disconnect_provider(
    provider: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Delete OAuth connection from database
    // TODO: Get user_id from session
    let user_id = 1; // Placeholder
    
    diesel::delete(
        crate::schema::oauth_connections::table
            .filter(crate::schema::oauth_connections::user_id.eq(user_id))
            .filter(crate::schema::oauth_connections::provider_id.eq(
                diesel::dsl::sql("(SELECT id FROM oauth_providers WHERE name = '")
                    .bind::<diesel::sql_types::Text, _>(provider.as_str())
                    .sql("')")
            ))
    )
    .execute(&conn)
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("{} disconnected", provider)
    })))
}
