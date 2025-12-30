use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::models::{DbPool, verification_models::*};
use crate::services::errors_service::CustomHttpError;
use crate::helpers::tenant_helper::resolve_tenant_id;
use diesel::prelude::*;

// ===== Request/Response DTOs =====

#[derive(Debug, Deserialize)]
pub struct UpdateVerificationSettingsRequest {
    pub ttl_hours: Option<i32>,
    pub enabled: Option<bool>,
    pub email_template: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerificationSettingsResponse {
    pub settings: VerificationSettings,
}

#[derive(Debug, Serialize)]
pub struct VerificationSettingsListResponse {
    pub settings: Vec<VerificationSettings>,
}

// ===== Admin Endpoints =====

/// Get verification settings for current tenant
#[utoipa::path(
    get,
    path = "/v1/api/verification/settings",
    tag = "Internal - Verification",
    params(
        ("verification_type" = Option<String>, Query, description = "Filter by verification type (crm_customer, user_registration, etc.)")
    ),
    responses(
        (status = 200, description = "Verification settings retrieved", body = VerificationSettingsListResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_verification_settings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req)?;
    let verification_type = query.get("verification_type");
    
    let settings = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
        })?;
        
        use crate::schema::verification_settings;
        
        let mut query = verification_settings::table
            .filter(verification_settings::tenant_id.eq(tenant_id))
            .into_boxed();
        
        if let Some(v_type) = verification_type {
            query = query.filter(verification_settings::verification_type.eq(v_type));
        }
        
        query
            .load::<VerificationSettings>(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(format!("Query failed: {}", e)))
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
    
    Ok(HttpResponse::Ok().json(VerificationSettingsListResponse { settings }))
}

/// Update or create verification settings
#[utoipa::path(
    put,
    path = "/v1/api/verification/settings/{verification_type}",
    tag = "Internal - Verification",
    params(
        ("verification_type" = String, Path, description = "Verification type to configure")
    ),
    request_body = UpdateVerificationSettingsRequest,
    responses(
        (status = 200, description = "Settings updated successfully", body = VerificationSettingsResponse),
        (status = 400, description = "Invalid TTL value (must be 1-168 hours)"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_verification_settings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    verification_type: web::Path<String>,
    body: web::Json<UpdateVerificationSettingsRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req)?;
    let v_type = verification_type.into_inner();
    
    // Validate TTL (1 hour to 7 days)
    if let Some(ttl) = body.ttl_hours {
        if ttl < 1 || ttl > 168 {
            return Err(CustomHttpError::BadRequest(
                "TTL must be between 1 and 168 hours (7 days)".into()
            ));
        }
    }
    
    let settings = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
        })?;
        
        use crate::schema::verification_settings;
        use diesel::dsl::now;
        
        // Try to find existing settings
        let existing = verification_settings::table
            .filter(verification_settings::tenant_id.eq(tenant_id))
            .filter(verification_settings::verification_type.eq(&v_type))
            .first::<VerificationSettings>(&mut conn)
            .optional()
            .map_err(|e| CustomHttpError::InternalServerError(format!("Query failed: {}", e)))?;
        
        if let Some(mut existing_settings) = existing {
            // Update existing
            let mut changeset = diesel::update(verification_settings::table)
                .filter(verification_settings::id.eq(existing_settings.id))
                .into_boxed();
            
            if let Some(ttl) = body.ttl_hours {
                changeset = changeset
                    .set(verification_settings::ttl_hours.eq(ttl))
                    .into_boxed();
            }
            
            if let Some(enabled) = body.enabled {
                changeset = changeset
                    .set(verification_settings::enabled.eq(enabled))
                    .into_boxed();
            }
            
            if let Some(template) = &body.email_template {
                changeset = changeset
                    .set(verification_settings::email_template.eq(template))
                    .into_boxed();
            }
            
            changeset
                .set(verification_settings::updated_at.eq(now))
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Update failed: {}", e)))?;
            
            // Fetch updated record
            verification_settings::table
                .filter(verification_settings::id.eq(existing_settings.id))
                .first::<VerificationSettings>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Fetch failed: {}", e)))
        } else {
            // Create new
            let new_settings = NewVerificationSettings {
                tenant_id: Some(tenant_id),
                verification_type: v_type.clone(),
                ttl_hours: body.ttl_hours.unwrap_or(12),
                enabled: body.enabled.unwrap_or(true),
                email_template: body.email_template.clone(),
            };
            
            diesel::insert_into(verification_settings::table)
                .values(&new_settings)
                .get_result::<VerificationSettings>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(format!("Insert failed: {}", e)))
        }
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
    
    Ok(HttpResponse::Ok().json(VerificationSettingsResponse { settings }))
}

/// Delete verification settings (revert to defaults)
#[utoipa::path(
    delete,
    path = "/v1/api/verification/settings/{verification_type}",
    tag = "Internal - Verification",
    params(
        ("verification_type" = String, Path, description = "Verification type to reset")
    ),
    responses(
        (status = 200, description = "Settings deleted, will use defaults"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_verification_settings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    verification_type: web::Path<String>,
) -> Result<HttpResponse, CustomHttpError> {
    let tenant_id = resolve_tenant_id(&req)?;
    let v_type = verification_type.into_inner();
    
    web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
        })?;
        
        use crate::schema::verification_settings;
        
        diesel::delete(verification_settings::table)
            .filter(verification_settings::tenant_id.eq(tenant_id))
            .filter(verification_settings::verification_type.eq(&v_type))
            .execute(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(format!("Delete failed: {}", e)))?;
        
        Ok::<(), CustomHttpError>(())
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Settings deleted. System will use default values (12h TTL)."
    })))
}
