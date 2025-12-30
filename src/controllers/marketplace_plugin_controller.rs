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
use crate::models::tenant_models::Tenant;
use crate::services::auth_service::Claims;
use crate::models::marketplace_plugin_models::NewTenantPlugin;

#[derive(Deserialize)]
pub struct InstallPluginRequest {
    pub plugin_id: i32,
}

/// List all marketplace plugins
#[utoipa::path(
    get,
    path = "/v1/plugins",
    tag = "Marketplace - Plugins",
    responses(
        (status = 200, description = "List of all marketplace plugins", body = Vec<MarketplacePlugin>),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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

/// Submit a new plugin to marketplace
#[utoipa::path(
    post,
    path = "/v1/plugins/submit",
    tag = "Marketplace - Plugins",
    request_body(content = String, description = "Multipart form with plugin file, name, version, price_cents", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Plugin submitted successfully (pending approval)"),
        (status = 400, description = "Missing required fields"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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

/// Install a plugin to tenant
#[utoipa::path(
    post,
    path = "/v1/plugins/install",
    tag = "Marketplace - Plugins",
    request_body = InstallPluginRequest,
    responses(
        (status = 201, description = "Plugin installed successfully"),
        (status = 200, description = "Plugin already installed"),
        (status = 400, description = "Invalid user ID"),
        (status = 403, description = "User not associated with tenant"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn install_plugin(
    _req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<InstallPluginRequest>,
    claims: Claims,
) -> impl Responder {
    let user_id = match claims.sub.parse::<i32>() {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid user ID"),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    
    // Resolve Tenant
    let tenant_id = match Tenant::get_tenant_id_for_user(user_id, &mut conn) {
        Ok(Some(tid)) => tid,
        Ok(None) => return HttpResponse::Forbidden().json("User not associated with a tenant"),
        Err(_) => return HttpResponse::InternalServerError().json("Tenant resolution error"),
    };

    use crate::schema::tenant_plugins;

    // Check if already installed
    let exists: i64 = tenant_plugins::table
        .filter(tenant_plugins::tenant_id.eq(tenant_id))
        .filter(tenant_plugins::plugin_id.eq(item.plugin_id))
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    if exists > 0 {
        return HttpResponse::Ok().json("Plugin already installed");
    }

    let new_install = NewTenantPlugin {
        tenant_id,
        plugin_id: item.plugin_id,
        status: "active".to_string(),
        settings: None,
    };

    match diesel::insert_into(tenant_plugins::table)
        .values(&new_install)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Created().json(format!("Plugin {} installed successfully", item.plugin_id)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Installation failed: {}", e)),
    }
}

#[derive(Deserialize)]
pub struct ApprovePluginRequest {
    pub plugin_id: i32,
    pub status: String, // 'active', 'rejected'
}

pub async fn approve_plugin(
    _req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<ApprovePluginRequest>,
    claims: Claims,
) -> impl Responder {
    if claims.role != "admin" { // Simple role check
        return HttpResponse::Forbidden().json("Admin access required");
    }

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::marketplace_plugins;

    let target_status = if item.status == "rejected" { "rejected" } else { "active" };

    match diesel::update(marketplace_plugins::table.find(item.plugin_id))
        .set(marketplace_plugins::status.eq(target_status))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(format!("Plugin {} status set to {}", item.plugin_id, target_status)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error updating plugin: {}", e)),
    }
}
