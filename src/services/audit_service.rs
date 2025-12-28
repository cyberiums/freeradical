use actix_web::web;
use diesel::prelude::*;
use crate::models::DbPool;
use crate::models::audit_models::{NewAuditLog, AuditLog};
use crate::schema::audit_logs;
use crate::services::errors_service::CustomHttpError;

pub struct AuditService;

impl AuditService {
    pub async fn log(
        pool: &web::Data<DbPool>,
        tenant_id: Option<i32>,
        user_id: i32,
        action: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        details: Option<serde_json::Value>,
        ip_address: Option<&str>,
    ) -> Result<(), CustomHttpError> {
        let new_log = NewAuditLog {
            tenant_id,
            user_id,
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(|s| s.to_string()),
            details,
            ip_address: ip_address.map(|s| s.to_string()),
        };

        let pool = pool.clone();
        web::block(move || {
            let mut conn = pool.get().map_err(|_| {
                CustomHttpError::InternalServerError("DB Connection Error".to_string())
            })?;

            diesel::insert_into(audit_logs::table)
                .values(&new_log)
                .execute(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))??;

        Ok(())
    }

    pub async fn list_by_tenant(
        pool: &web::Data<DbPool>,
        tenant_id: i32,
    ) -> Result<Vec<AuditLog>, CustomHttpError> {
        let pool = pool.clone();
        let logs = web::block(move || {
            let mut conn = pool.get().map_err(|_| {
                CustomHttpError::InternalServerError("DB Connection Error".to_string())
            })?;

            audit_logs::table
                .filter(audit_logs::tenant_id.eq(tenant_id))
                .order(audit_logs::created_at.desc())
                .limit(100)
                .load::<AuditLog>(&mut conn)
                .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))
        })
        .await
        .map_err(|e| CustomHttpError::InternalServerError(e.to_string()))??;

        Ok(logs)
    }
}
