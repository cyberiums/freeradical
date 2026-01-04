use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::DatabasePool;
use crate::services::auth_service::create_jwt_token;
use chrono::{Utc, Duration};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::refresh_tokens)]
pub struct NewRefreshToken {
    pub user_id: i32,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::refresh_tokens)]
pub struct RefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub revoked_at: Option<chrono::NaiveDateTime>,
}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Successfully refreshed token", body = RefreshTokenResponse),
        (status = 401, description = "Invalid or expired refresh token"),
        (status = 500, description = "Server error")
    )
)]
pub async fn refresh_token(
    req: web::Json<RefreshTokenRequest>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    use crate::schema::{refresh_tokens, users};

    // Find and validate refresh token
    let token_record = refresh_tokens::table
        .filter(refresh_tokens::token.eq(&req.refresh_token))
        .filter(refresh_tokens::revoked_at.is_null())
        .first::<RefreshToken>(&mut conn)
        .optional()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let token_data = match token_record {
        Some(record) => record,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid refresh token"
            })));
        }
    };

    // Check if expired
    let now = Utc::now().naive_utc();
    if token_data.expires_at < now {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Refresh token expired"
        })));
    }

    // Get user info
    let user = users::table
        .filter(users::id.eq(token_data.user_id))
        .select((users::id, users::username))
        .first::<(i32, String)>(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("User not found: {}", e)))?;

    let (uid, email) = user;

    // Generate new access token (24 hours)
    let new_access_token = create_jwt_token(uid, email.clone(), "user".to_string())
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create JWT: {}", e)))?;

    // Rotate refresh token (generate new one, revoke old one)
    let new_refresh_token = Uuid::new_v4().to_string();
    let new_expires_at = now + Duration::days(30);

    // Revoke old refresh token
    diesel::update(refresh_tokens::table.filter(refresh_tokens::id.eq(token_data.id)))
        .set(refresh_tokens::revoked_at.eq(Some(now)))
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to revoke old token: {}", e)))?;

    // Insert new refresh token
    diesel::insert_into(refresh_tokens::table)
        .values(&NewRefreshToken {
            user_id: uid,
            token: new_refresh_token.clone(),
            expires_at: new_expires_at,
        })
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create new refresh token: {}", e)))?;

    Ok(HttpResponse::Ok().json(RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    }))
}

/// Revoke refresh token (logout)
#[utoipa::path(
    post,
    path = "/v1/auth/revoke",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token revoked successfully"),
        (status = 500, description = "Server error")
    )
)]
pub async fn revoke_token(
    req: web::Json<RefreshTokenRequest>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    use crate::schema::refresh_tokens;

    let now = Utc::now().naive_utc();

    // Revoke the refresh token
    diesel::update(
        refresh_tokens::table
            .filter(refresh_tokens::token.eq(&req.refresh_token))
            .filter(refresh_tokens::revoked_at.is_null())
    )
    .set(refresh_tokens::revoked_at.eq(Some(now)))
    .execute(&mut conn)
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to revoke token: {}", e)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Token revoked successfully"
    })))
}
