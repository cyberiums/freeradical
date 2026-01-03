use actix_web::{get, post, HttpResponse, Responder};
use crate::services::backup_service::BackupService;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct BackupList {
    backups: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BackupResponse {
    success: bool,
    message: String,
}

/// List all backups
#[utoipa::path(
    get,
    path = "/admin/backups",
    tag = "Internal - Backup",
    responses(
        (status = 200, description = "List of backups", body = BackupList)
    )
)]
#[get("/admin/backups")]
pub async fn list_backups() -> impl Responder {
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    
    // Ensure exists
    if let Err(_) = std::fs::create_dir_all(&backup_dir) {
        return HttpResponse::InternalServerError().finish();
    }

    match BackupService::list_backups(&backup_dir) {
        Ok(files) => HttpResponse::Ok().json(BackupList { backups: files }),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

/// Create a new backup
#[utoipa::path(
    post,
    path = "/admin/backups",
    tag = "Internal - Backup",
    responses(
        (status = 200, description = "Backup created", body = BackupResponse),
        (status = 500, description = "Backup failed")
    )
)]
#[post("/admin/backups")]
pub async fn create_backup() -> impl Responder {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_URL not set"),
    };
    
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    
    // Create
    let backup_path = match BackupService::create_backup(&db_url, &backup_dir) {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().json(BackupResponse { 
            success: false, 
            message: format!("Backup failed: {}", e) 
        }),
    };
    
    // Compress
    match BackupService::compress_backup(&backup_path) {
        Ok(_) => {
            // Remove original uncompressed file to save space
            let _ = std::fs::remove_file(backup_path);
            HttpResponse::Ok().json(BackupResponse {
                success: true,
                message: "Backup created successfully".to_string()
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(BackupResponse {
            success: false,
            message: format!("Compression failed: {}", e)
        }),
    }
}
