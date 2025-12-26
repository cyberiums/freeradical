use actix_web::{web, HttpResponse};
use crate::services::backup_service::BackupService;
use crate::services::errors_service::CustomHttpError;
use crate::services::auth_service::Claims;
use crate::models::DatabasePool;

pub async fn create_backup(
    _pool: web::Data<DatabasePool>,
    _claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let db_url = std::env::var("DATABASE_URL")
        .map_err(|e| CustomHttpError::InternalServerError(format!("DATABASE_URL not set: {}", e)))?;
    
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    
    // Ensure backup directory exists
    std::fs::create_dir_all(&backup_dir)
        .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to create backup dir: {}", e)))?;
    
    let backup_path = BackupService::create_backup(&db_url, &backup_dir)
        .map_err(|e| {
            log::error!("Backup failed: {}", e);
            CustomHttpError::InternalServerError(format!("Backup failed: {}", e))
        })?;
    
    let compressed_path = BackupService::compress_backup(&backup_path)
        .map_err(|e| {
            log::error!("Compression failed: {}", e);
            CustomHttpError::InternalServerError(format!("Compression failed: {}", e))
        })?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "backup_file": compressed_path
    })))
}
