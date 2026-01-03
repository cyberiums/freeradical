use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::fs;
use uuid::Uuid;
use crate::models::theme_models::{NewTheme, Theme};
use crate::models::db_connection;
use crate::middleware::auth_middleware::get_user_context;
use diesel::prelude::*;

/// List all available themes
#[utoipa::path(
    get,
    path = "/v1/themes",
    tag = "Marketplace - Themes",
    responses(
        (status = 200, description = "List of all themes", body = Vec<Theme>)
    )
)]
pub async fn list_themes(
    _req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::themes;

    let results = themes::table
        .load::<Theme>(&mut conn);

    match results {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error listing themes: {}", e)),
    }
}

/// Upload a new theme
#[utoipa::path(
    post,
    path = "/v1/themes/upload",
    tag = "Marketplace - Themes",
    request_body(content = String, description = "Multipart form with theme file, name, version", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Theme uploaded successfully"),
        (status = 400, description = "Missing required fields"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn upload_theme(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    mut payload: Multipart,
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    // Prepare variables to collect form data
    let mut theme_name = String::new();
    let mut theme_version = String::new();
    let mut file_path = String::new();
    
    // Iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().json("Error reading multipart field"),
        };
        // Fix: content_disposition() returns Option in some versions or compiler is picky
        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => return HttpResponse::BadRequest().json("Missing content disposition"),
        };
        let field_name = content_disposition.get_name().unwrap_or("").to_string();

        if field_name == "theme_file" {
             let filename = content_disposition
                .get_filename()
                .map(|f| {
                    let ext = std::path::Path::new(f).extension().and_then(|s| s.to_str()).unwrap_or("zip");
                    format!("{}.{}", Uuid::new_v4(), ext)
                })
                .unwrap_or_else(|| Uuid::new_v4().to_string() + ".zip");
            
            let upload_dir = "./uploads/themes";
            fs::create_dir_all(upload_dir).ok();
            let filepath = format!("{}/{}", upload_dir, filename);
            let mut f = fs::File::create(&filepath).expect("Failed to create file");

            // Field is a stream of bytes
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).unwrap();
            }
            file_path = filepath;
        } else {
             // Read text fields (name, version)
             let mut value_bytes = Vec::new();
             while let Some(chunk) = field.next().await {
                 let data = chunk.unwrap();
                 value_bytes.extend_from_slice(&data);
             }
             let value = String::from_utf8(value_bytes).unwrap_or_default();
             
             match field_name.as_str() {
                 "name" => theme_name = value,
                 "version" => theme_version = value,
                 _ => {}
             }
        }
    }

    if theme_name.is_empty() || file_path.is_empty() {
        return HttpResponse::BadRequest().json("Name and theme_file are required");
    }

    // Insert into DB
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::themes;

    let new_theme = NewTheme {
        name: theme_name,
        description: None,
        version: if theme_version.is_empty() { "1.0.0".to_string() } else { theme_version },
        file_path: file_path,
        thumbnail_url: None,
        is_active: Some(false),
        is_default: Some(false),
        tenant_id: None, 
    };

    match diesel::insert_into(themes::table).values(&new_theme).execute(&mut conn) {
        Ok(_) => HttpResponse::Created().json("Theme uploaded successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error saving theme: {}", e)),
    }
}

/// Activate a theme
#[utoipa::path(
    post,
    path = "/v1/themes/{id}/activate",
    tag = "Marketplace - Themes",
    params(
        ("id" = i32, Path, description = "Theme ID to activate", example = 5)
    ),
    responses(
        (status = 200, description = "Theme activated successfully"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn activate_theme(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
     let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let theme_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::themes;

    // Logic: Deactivate all others (if tenant scoped), activate this one.
    // Simpler: Just set is_active = true for this one.

    // 1. Deactivate others (optional, depending on one-theme-per-site rule)
    // diesel::update(themes::table).set(themes::is_active.eq(false)).execute(&mut conn)?;

    let res = diesel::update(themes::table.find(theme_id))
        .set(themes::is_active.eq(true))
        .execute(&mut conn);

    match res {
        Ok(_) => HttpResponse::Ok().json("Theme activated"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error activating theme: {}", e)),
    }
}
