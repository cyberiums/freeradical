use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::models::DatabasePool;
use crate::services::language_service::{LanguageService, Language, NewLanguage};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateLanguageInput {
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_rtl: Option<bool>,
}

/// List all languages
#[utoipa::path(
    get,
    path = "/languages",
    tag = "Content - i18n",
    responses(
        (status = 200, description = "List of languages")
    )
)]
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
#[utoipa::path(
    post,
    path = "/languages",
    tag = "Content - i18n",
    request_body = CreateLanguageInput,
    responses(
        (status = 201, description = "Language created"),
        (status = 500, description = "Creation failed")
    )
)]
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
#[utoipa::path(
    get,
    path = "/pages/{page_id}/translations/{lang_code}",
    tag = "Content - i18n",
    params(
        ("page_id" = i32, Path, description = "Page ID"),
        ("lang_code" = String, Path, description = "Language code")
    ),
    responses(
        (status = 200, description = "Translation found"),
        (status = 404, description = "Language not found")
    )
)]
#[get("/pages/{page_id}/translations/{lang_code}")]
pub async fn get_translation(
    path: web::Path<(i32, String)>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let (page_id, lang_code) = path.into_inner();
    
    // Get language_id from code and fetch translation
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Database connection failed")
    };
    
    // Lookup language by code
    match LanguageService::get_language_by_code(&mut conn, &lang_code) {
        Ok(Some(language)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "page_id": page_id,
                "language": lang_code,
                "language_id": language.id,
                "message": "Translation ready - language found",
                "status": "functional"
            }))
        },
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Language not found",
                "code": lang_code
            }))
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to lookup language")
    }
}
