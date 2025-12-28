use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::models::db_connection::DatabasePool;
use crate::middleware::auth_middleware::get_user_context;
use crate::models::webhook_models::{NewTenantWebhook, TenantWebhook};
use crate::schema::tenant_webhooks;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
}

pub async fn list_webhooks(
    req: HttpRequest,
    pool: web::Data<DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let tenant_id = path.into_inner();
    let mut conn = pool.get().expect("db conn");

    let items = tenant_webhooks::table
        .filter(tenant_webhooks::tenant_id.eq(tenant_id))
        .load::<TenantWebhook>(&mut conn);

    match items {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn create_webhook(
    req: HttpRequest,
    pool: web::Data<DatabasePool>,
    path: web::Path<i32>,
    body: web::Json<CreateWebhookRequest>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let tenant_id = path.into_inner();
    let mut conn = pool.get().expect("db conn");

    let new_hook = NewTenantWebhook {
        id: Uuid::new_v4(),
        tenant_id,
        url: body.url.clone(),
        secret: body.secret.clone(),
        events: serde_json::to_value(&body.events).unwrap(),
        is_active: true,
    };

    let res = diesel::insert_into(tenant_webhooks::table)
        .values(&new_hook)
        .get_result::<TenantWebhook>(&mut conn);

    match res {
        Ok(hook) => HttpResponse::Created().json(hook),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn delete_webhook(
    req: HttpRequest,
    pool: web::Data<DatabasePool>,
    path: web::Path<(i32, Uuid)>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let (tenant_id, hook_id) = path.into_inner();
    let mut conn = pool.get().expect("db conn");

    let res = diesel::delete(tenant_webhooks::table)
        .filter(tenant_webhooks::id.eq(hook_id))
        .filter(tenant_webhooks::tenant_id.eq(tenant_id))
        .execute(&mut conn);

    match res {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
