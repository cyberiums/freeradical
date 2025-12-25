use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::DatabasePool;
use crate::services::language_service::{LanguageService, Language, NewLanguage};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLanguageInput {
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_rtl: Option<bool>,
}

/// List all languages
#[get("/languages")]
pub async fn list_languages(pool: web::Data<DatabasePool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Database connection failed")
    };
    
    match LanguageService::get_enabled_languages(&mut conn) {
        Ok(languages) => HttpResponse::Ok().json(languages),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch languages")
    }
}

/// Create a new language
#[post("/languages")]
pub async fn create_language(
    input: web::Json<CreateLanguageInput>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Database connection failed")
    };
    
    let new_lang = NewLanguage {
        code: input.code.clone(),
        name: input.name.clone(),
        native_name: input.native_name.clone(),
        is_default: Some(false),
        is_rtl: input.is_rtl,
        enabled: Some(true),
    };
    
    match LanguageService::create_language(&mut conn, new_lang) {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "code": input.code,
            "name": input.name,
            "message": "Language created successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create language")
    }
}

/// Get translation for a page
#[get("/pages/{page_id}/translations/{lang_code}")]
pub async fn get_translation(
    path: web::Path<(i32, String)>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let (page_id, lang_code) = path.into_inner();
    
    // TODO: Get language_id from code, then fetch translation
    HttpResponse::Ok().json(serde_json::json!({
        "page_id": page_id,
        "language": lang_code,
        "message": "Translation retrieval - implement language code lookup"
    }))
}
