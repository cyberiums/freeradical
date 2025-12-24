// Simplified Media Controller
// Iteration 4, Task 1 - Basic implementation

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db_connection::establish_connection;
use crate::models::media_models::{Media, NewMedia};
use crate::schema::media;

/// List all media files
/// GET /api/media
pub async fn list_media() -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = establish_connection();
    
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
/// GET /api/media/:uuid
pub async fn get_media(media_uuid: web::Path<String>) -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = establish_connection();
    
    match media
        .filter(uuid.eq(media_uuid.as_str()))
        .first::<Media>(&mut conn)
    {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().json("Media not found"),
    }
}

/// Delete media file
/// DELETE /api/media/:uuid
pub async fn delete_media(media_uuid: web::Path<String>) -> impl Responder {
    use crate::schema::media::dsl::*;
    
    let mut conn = establish_connection();
    
    match diesel::delete(media.filter(uuid.eq(media_uuid.as_str())))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Media deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete media"),
    }
}

/// Basic media upload (simplified - no actual file handling yet)
/// POST /api/media/upload
/// This is a placeholder that creates a database entry
/// Full implementation would handle multipart uploads
pub async fn upload_media(info: web::Json<NewMedia>) -> impl Responder {
    let mut conn = establish_connection();
    
    let new_media = NewMedia {
        uuid: Uuid::new_v4().to_string(),
        filename: info.filename.clone(),
        original_filename: info.original_filename.clone(),
        mime_type: info.mime_type.clone(),
        file_size: info.file_size,
        width: info.width,
        height: info.height,
        folder: info.folder.clone(),
        storage_path: info.storage_path.clone(),
        cdn_url: None,
        upload_user_id: None,
        alt_text: info.alt_text.clone(),
        caption: info.caption.clone(),
    };
    
    match diesel::insert_into(media::table)
        .values(&new_media)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Created().json("Media uploaded"),
        Err(e) => HttpResponse::InternalServerError()
            .json(format!("Failed to save media: {}", e)),
    }
}
