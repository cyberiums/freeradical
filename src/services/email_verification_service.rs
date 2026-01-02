use actix_web::web;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rand::{thread_rng, Rng, distr::Alphanumeric};
use uuid::Uuid;

use crate::models::DbPool;
use crate::models::verification_models::{NewPendingVerification, PendingVerification, VerificationSettings};
use crate::services::errors_service::CustomHttpError;
use crate::services::email_service::EmailService;
use crate::schema::{pending_verifications, verification_settings};

pub struct EmailVerificationService;

impl EmailVerificationService {
    /// Create pending verification and send email
    pub async fn create_and_send(
        pool: &DbPool,
        verification_type: &str,
        email: &str,
        payload: serde_json::Value,
        tenant_id: Option<i32>,
        email_service: &EmailService,
    ) -> Result<String, CustomHttpError> {
        // 1. Get TTL from settings
        let ttl_hours = Self::get_ttl_hours(pool, verification_type, tenant_id).await?;
        
        // 2. Generate secure token
        let token = Self::generate_token();
        
        // 3. Calculate expiration
        let expires_at = Utc::now() + Duration::hours(ttl_hours as i64);
        
        // 4. Create verification record
        let new_verification = NewPendingVerification {
            uuid: Uuid::new_v4(),
            verification_type: verification_type.to_string(),
            email: email.to_string(),
            verification_token: token.clone(),
            payload,
            tenant_id,
            expires_at: expires_at.naive_utc(),
        };
        
        // 5. Insert into database
        let pool_clone = pool.clone();
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
            })?;
            
            diesel::insert_into(pending_verifications::table)
                .values(&new_verification)
                .execute(&mut conn)
                .map_err(|e| {
                    CustomHttpError::InternalServerError(format!("Failed to create verification: {}", e))
                })?;
            
            Ok::<(), CustomHttpError>(())
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
        
        // 6. Send verification email
        Self::send_verification_email(email, &token, verification_type, ttl_hours, email_service).await?;
        
        Ok(token)
    }
    
    /// Verify token and return payload
    pub async fn verify_and_get_payload(
        pool: &DbPool,
        token: &str,
    ) -> Result<(String, serde_json::Value, Option<i32>), CustomHttpError> {
        let token = token.to_string();
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
            })?;
            
            // Find verification by token
            let verification: PendingVerification = pending_verifications::table
                .filter(pending_verifications::verification_token.eq(&token))
                .first(&mut conn)
                .map_err(|_| {
                    CustomHttpError::NotFound("Invalid or expired verification token".into())
                })?;
            
            // Check if already verified
            if verification.verified.unwrap_or(false) {
                return Err(CustomHttpError::BadRequest("Token already used".into()));
            }
            
            // Check expiration
            let now = Utc::now().naive_utc();
            if verification.expires_at < now {
                return Err(CustomHttpError::BadRequest("Verification token expired".into()));
            }
            
            // Mark as verified
            diesel::update(pending_verifications::table)
                .filter(pending_verifications::verification_token.eq(&token))
                .set((
                    pending_verifications::verified.eq(true),
                    pending_verifications::verified_at.eq(Some(now)),
                ))
                .execute(&mut conn)
                .map_err(|e| {
                    CustomHttpError::InternalServerError(format!("Failed to mark verified: {}", e))
                })?;
            
            Ok::<(String, serde_json::Value, Option<i32>), CustomHttpError>((
                verification.email,
                verification.payload,
                verification.tenant_id,
            ))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Background job: cleanup expired verifications
    pub async fn cleanup_expired(pool: &DbPool) -> Result<usize, CustomHttpError> {
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
            })?;
            
            let now = Utc::now().naive_utc();
            
            // Delete expired, unverified records
            diesel::delete(pending_verifications::table)
                .filter(pending_verifications::expires_at.lt(now))
                .filter(pending_verifications::verified.eq(Some(false)))
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Delete error: {}", e)))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    }
    
    /// Generate secure random token
    fn generate_token() -> String {
        let uuid_part = Uuid::new_v4().to_string();
        let random_part: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        
        format!("{}-{}", uuid_part, random_part)
    }
    
    /// Get TTL hours from settings
    async fn get_ttl_hours(
        pool: &DbPool,
        verification_type: &str,
        tenant_id: Option<i32>,
    ) -> Result<i32, CustomHttpError> {
        let verification_type = verification_type.to_string();
        let pool_clone = pool.clone();
        
        web::block(move || {
            let mut conn = pool_clone.get().map_err(|e| {
                CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
            })?;
            
            // Try tenant-specific setting first
            if let Some(tid) = tenant_id {
                if let Ok(settings) = verification_settings::table
                    .filter(verification_settings::tenant_id.eq(tid))
                    .filter(verification_settings::verification_type.eq(&verification_type))
                    .filter(verification_settings::enabled.eq(Some(true)))
                    .first::<VerificationSettings>(&mut conn)
                {
                    return Ok(settings.ttl_hours.unwrap_or(12));
                }
            }
            
            // Fall back to global default (tenant_id IS NULL)
            verification_settings::table
                .filter(verification_settings::tenant_id.is_null())
                .filter(verification_settings::verification_type.eq(&verification_type))
                .filter(verification_settings::enabled.eq(Some(true)))
                .first::<VerificationSettings>(&mut conn)
                .map(|settings| settings.ttl_hours.unwrap_or(12))
                .or(Ok(12)) // Hard default: 12 hours
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to get TTL: {}", e)))?
    }
    
    /// Send verification email
    async fn send_verification_email(
        email: &str,
        token: &str,
        verification_type: &str,
        ttl_hours: i32,
        email_service: &EmailService,
    ) -> Result<(), CustomHttpError> {
        let base_url = std::env::var("APP_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8000".to_string());
        let verification_link = format!("{}/v1/public/crm/verify/{}", base_url, token);
        
        let subject = match verification_type {
            "crm_customer" => "Verify your email address",
            "user_registration" => "Confirm your account",
            "form_submission" => "Verify your form submission",
            _ => "Email verification required",
        };
        
        let body = format!(
            r#"
            <html>
            <body style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2>Verify Your Email</h2>
                <p>Thank you for signing up! Please click the button below to verify your email address:</p>
                <a href="{}" 
                   style="display: inline-block; padding: 12px 24px; background: #667eea; color: white; 
                          text-decoration: none; border-radius: 4px; margin: 20px 0; font-weight: bold;">
                    Verify Email
                </a>
                <p style="color: #666; font-size: 14px;">
                    <strong>This link expires in {} hours.</strong>
                </p>
                <p style="color: #666; font-size: 14px;">
                    If you didn't request this, please ignore this email.
                </p>
                <hr style="margin: 30px 0; border: none; border-top: 1px solid #eee;">
                <p style="color: #999; font-size: 12px;">
                    Powered by FreeRadical CMS
                </p>
            </body>
            </html>
            "#,
            verification_link, ttl_hours
        );
        
        email_service.send_email(email, subject, &body).await
            .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to send email: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_token() {
        let token1 = EmailVerificationService::generate_token();
        let token2 = EmailVerificationService::generate_token();
        
        // Tokens should be different
        assert_ne!(token1, token2);
        
        // Token should have reasonable length (UUID + hyphen + 32 chars)
        assert!(token1.len() > 50);
    }
}
