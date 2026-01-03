use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use utoipa::ToSchema;
use diesel::prelude::*;
use futures_util::StreamExt;
use std::path::PathBuf;
use uuid::Uuid;

use crate::models::media_models::{Media, NewMedia};
use crate::schema::media;
use crate::services::database_service;
use crate::services::image_service;
use crate::services::storage_service::{StorageService, StorageBackend};

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// List all media files
#[utoipa::path(
    get,
    path = "/api/media",
    tag = "Content - Media",
    responses(
        (status = 200, description = "List of media files (max 100)")
    )
)]
pub async fn list_media() -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = database_service::establish_connection();
    
    match media
        .order(created_at.desc())
        .limit(100)
        .load::<Media>(&mut conn)
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch media"),
    }
}

/// Get single media file
#[utoipa::path(
    get,
    path = "/api/media/{uuid}",
    tag = "Content - Media",
    params(
        ("uuid" = String, Path, description = "Media UUID")
    ),
    responses(
        (status = 200, description = "Media file details"),
        (status = 404, description = "Media not found")
    )
)]
pub async fn get_media(media_uuid: web::Path<String>) -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = database_service::establish_connection();
    
    match media
        .filter(uuid.eq(media_uuid.as_str()))
        .first::<Media>(&mut conn)
    {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().json("Media not found"),
    }
}

/// Delete media file
#[utoipa::path(
    delete,
    path = "/api/media/{uuid}",
    tag = "Content - Media",
    params(
        ("uuid" = String, Path, description = "Media UUID")
    ),
    responses(
        (status = 200, description = "Media deleted"),
        (status = 404, description = "Media not found")
    )
)]
pub async fn delete_media(
    media_uuid: web::Path<String>,
    storage: web::Data<StorageBackend>,
) -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = database_service::establish_connection();
    
    // Get media to find file path
    let media_item = match media
        .filter(uuid.eq(media_uuid.as_str()))
        .first::<Media>(&mut conn)
    {
        Ok(item) => item,
        Err(_) => return HttpResponse::NotFound().json("Media not found"),
    };
    
    // Delete database record
    match diesel::delete(media.filter(uuid.eq(media_uuid.as_str())))
        .execute(&mut conn)
    {
        Ok(_) => {
            // Delete file from storage
            // Extract filename from file_path or store simple filename in DB? 
            // Currently file_path stores "uploads/filename". S3 implementation expects "filename".
            // We should ideally store the key/filename in the DB.
            // For now, let's assume file_path might be a full path or just filename.
            // But verify: upload_media stores `storage_path`.
            
            // FIXME: The DB stores `file_path`. If it was "uploads/uuid.ext", 
            // we need to extract "uuid.ext" for S3.
            let target_filename = std::path::Path::new(&media_item.file_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
                
            let _ = storage.delete(target_filename).await;
            HttpResponse::Ok().json("Media deleted")
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete media"),
    }
}

/// Upload media file (multipart form data)
#[utoipa::path(
    post,
    path = "/api/media/upload",
    tag = "Content - Media",
    request_body(content = String, description = "Multipart form with file, alt_text, caption, folder", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "File uploaded successfully"),
        (status = 400, description = "Invalid file or upload failed"),
        (status = 413, description = "File too large")
    )
)]
pub async fn upload_media(
    req: actix_web::HttpRequest, 
    mut payload: Multipart,
    storage: web::Data<StorageBackend>,
) -> impl Responder {
    let mut filename = String::new();
    let mut file_data: Vec<u8> = Vec::new();
    let mut alt_text = None;
    let mut caption = None;
    let mut folder = None;
    
    // Process multipart fields
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().json("Invalid multipart data"),
        };
        
        // In actix-multipart v0.7, content_disposition() was removed
        // Field metadata is accessed differently
        let field_name = field.name();
        
        match field_name {
            Some("file") => {
                // Get original filename from field
                filename = match field.content_disposition().and_then(|cd| cd.get_filename()) {
                    Some(name) => name.to_string(),
                    None => "unknown".to_string(),
                };
                
                // Stream file data
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(data) => data,
                        Err(_) => return HttpResponse::BadRequest().json("Failed to read file"),
                    };
                    
                    file_data.extend_from_slice(&data);
                    
                    // Check file size limit
                    if file_data.len() > MAX_FILE_SIZE {
                        return HttpResponse::PayloadTooLarge()
                            .json(format!("File too large (max {}MB)", MAX_FILE_SIZE / 1024 / 1024));
                    }
                }
            }
            Some("alt_text") => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(data) = chunk {
                        value.push_str(&String::from_utf8_lossy(&data));
                    }
                }
                if !value.is_empty() {
                    alt_text = Some(value);
                }
            }
            Some("caption") => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(data) = chunk {
                        value.push_str(&String::from_utf8_lossy(&data));
                    }
                }
                if !value.is_empty() {
                    caption = Some(value);
                }
            }
            Some("folder") => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(data) = chunk {
                        value.push_str(&String::from_utf8_lossy(&data));
                    }
                }
                if !value.is_empty() {
                    folder = Some(value);
                }
            }
            _ => {}  // Ignore unknown fields
        }
    }
    
    // Validate file was provided
    if file_data.is_empty() {
        return HttpResponse::BadRequest().json("No file provided");
    }
    
    // Detect MIME type
    let mime_type = match infer::get(&file_data) {
        Some(kind) => kind.mime_type().to_string(),
        None => "application/octet-stream".to_string(),
    };
    
    // Validate image type (for MVP only allow images)
    if !mime_type.starts_with("image/") {
        return HttpResponse::BadRequest()
            .json(format!("Invalid file type. Only images allowed. Got: {}", mime_type));
    }
    
    // Extract image dimensions
    let (width, height) = match image::load_from_memory(&file_data) {
        Ok(img) => (Some(img.width() as i32), Some(img.height() as i32)),
        Err(_) => (None, None),  // Not an image or unreadable
    };
    
    // Generate unique filename with original extension
    let filename_path = PathBuf::from(&filename);
    let ext = filename_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("bin");
    let new_uuid = Uuid::new_v4().to_string();
    let new_filename = format!("{}.{}", new_uuid, ext);
    
    // Optimize images (resize, compress, generate WebP)
    let (optimized_data, webp_data, opt_width, opt_height) = if mime_type.starts_with("image/") {
        match image_service::optimize_image(&file_data, 1920, true) {
            Ok((jpeg_bytes, webp_bytes, w, h)) => {
                log::info!("Image optimized: {}x{}, original: {} bytes, optimized: {} bytes", 
                    w, h, file_data.len(), jpeg_bytes.len());
                (jpeg_bytes, webp_bytes, Some(w as i32), Some(h as i32))
            }
            Err(e) => {
                log::warn!("Image optimization failed: {}, using original", e);
                (file_data.clone(), None, width, height)
            }
        }
    } else {
        (file_data.clone(), None, width, height)
    };
    
    // Save optimized file to storage
    let storage_path = match storage.save(&new_filename, &optimized_data, &mime_type).await {
        Ok(path) => path, // For S3 this is the key, for Local it is uploads/filename
        Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to save file: {}", e)),
    };
    
    // Save WebP variant if generated
    let webp_path = match webp_data {
        Some(webp_bytes) => {
            let webp_filename = format!("{}.webp", new_uuid);
            match storage.save(&webp_filename, &webp_bytes, "image/webp").await {
                Ok(path) => {
                    log::info!("WebP variant saved: {}", webp_filename);
                    Some(path)
                }
                Err(_) => None,
            }
        }
        None => None,
    };
    
    // Create database record
    let new_media = NewMedia {
        uuid: new_uuid.clone(),
        filename: new_filename.clone(),
        original_filename: filename.clone(),
        file_path: storage_path.clone(),
        mime_type: mime_type.clone(),
        file_size: optimized_data.len() as i64,  // Size of optimized file
        width: opt_width,
        height: opt_height,
        alt_text,
        title: None,
        description: None,
        tags: None,
        uploaded_by: crate::middleware::auth_middleware::get_user_context(&req)
            .map(|ctx| Some(ctx.user_id))
            .unwrap_or(None),
    };
    
    let mut conn = database_service::establish_connection();
    
    match diesel::insert_into(media::table)
        .values(&new_media)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "uuid": new_uuid,
            "filename": new_filename,
            "original_filename": filename,
            "mime_type": mime_type,
            "file_size": optimized_data.len(),
            "original_size": file_data.len(),
            "width": opt_width,
            "height": opt_height,
            "storage_path": storage_path,
            "webp_generated": webp_path.is_some(),
            "webp_path": webp_path,
            "message": "File uploaded and optimized successfully"
        })),
        Err(e) => {
            // Delete files if database insert fails
            let _ = storage.delete(&new_filename).await;
            if let Some(_) = webp_path {
                 let webp_filename = format!("{}.webp", new_uuid);
                let _ = storage.delete(&webp_filename).await;
            }
            HttpResponse::InternalServerError()
                .json(format!("Failed to save media metadata: {}", e))
        }
    }
}
