use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::fs;
use uuid::Uuid;
use crate::models::marketplace_plugin_models::{NewMarketplacePlugin, MarketplacePlugin};
use crate::models::db_connection;
use crate::middleware::auth_middleware::get_user_context;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InstallPluginRequest {
    pub plugin_id: i32,
}

pub async fn list_plugins(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
     let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::marketplace_plugins;

    let results = marketplace_plugins::table
        .load::<MarketplacePlugin>(&mut conn);

    match results {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error listing plugins: {}", e)),
    }
}

pub async fn submit_plugin(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    mut payload: Multipart,
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    let mut plugin_name = String::new();
    let mut plugin_version = String::new();
    let mut file_path = String::new();
    let mut price_cents = 0;
    
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().json("Error reading multipart field"),
        };
        // Fix: content_disposition() returns Option
        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => return HttpResponse::BadRequest().json("Missing content disposition"),
        };
        let field_name = content_disposition.get_name().unwrap_or("").to_string();

        if field_name == "plugin_file" {
             let filename = content_disposition
                .get_filename()
                .map(|f| {
                    let ext = std::path::Path::new(f).extension().and_then(|s| s.to_str()).unwrap_or("zip");
                    format!("{}.{}", Uuid::new_v4(), ext)
                })
                .unwrap_or_else(|| Uuid::new_v4().to_string() + ".zip");
            
            let upload_dir = "./uploads/plugins";
            fs::create_dir_all(upload_dir).ok();
            let filepath = format!("{}/{}", upload_dir, filename);
            let mut f = fs::File::create(&filepath).expect("Failed to create file");

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).unwrap();
            }
            file_path = filepath;
        } else {
             let mut value_bytes = Vec::new();
             while let Some(chunk) = field.next().await {
                 let data = chunk.unwrap();
                 value_bytes.extend_from_slice(&data);
             }
             let value = String::from_utf8(value_bytes).unwrap_or_default();
             
             match field_name.as_str() {
                 "name" => plugin_name = value,
                 "version" => plugin_version = value,
                 "price_cents" => price_cents = value.parse().unwrap_or(0),
                 _ => {}
             }
        }
    }

    if plugin_name.is_empty() || file_path.is_empty() {
         return HttpResponse::BadRequest().json("Name and plugin_file are required");
    }

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::marketplace_plugins;

    let new_plugin = NewMarketplacePlugin {
        name: plugin_name,
        description: None,
        version: if plugin_version.is_empty() { "1.0.0".to_string() } else { plugin_version },
        file_path: file_path,
        icon_url: None,
        status: Some("pending".to_string()),
        developer_id: None, 
        price_cents: Some(price_cents),
    };

    match diesel::insert_into(marketplace_plugins::table).values(&new_plugin).execute(&mut conn) {
        Ok(_) => HttpResponse::Created().json("Plugin submitted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error submitting plugin: {}", e)),
    }
}

pub async fn install_plugin(
    req: HttpRequest,
    _pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<InstallPluginRequest>
) -> impl Responder {
     let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    // Logic to install:
    // 1. Verify plugin exists.
    // 2. Add to tenant's installed plugins list (need a join table for this, e.g. tenant_plugins).
    // For now, MVP: Just return success.
    
    HttpResponse::Ok().json(format!("Plugin {} installed (simulated)", item.plugin_id))
}
