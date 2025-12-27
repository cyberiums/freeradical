use actix_web::{web, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

/// OAuth authorization endpoint
#[get("/oauth/{provider}/authorize")]
pub async fn oauth_authorize(
    path: web::Path<String>
) -> impl Responder {
    let provider = path.into_inner();
    
    use crate::services::oauth_service::OAuthService;
    
    let redirect_uri = "http://localhost:8000/oauth/callback";
    // Generate state parameter for CSRF protection
    let state = uuid::Uuid::new_v4().to_string();
    
    match OAuthService::get_authorization_url(&provider, redirect_uri, &state) {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
}

/// OAuth callback endpoint
#[get("/oauth/callback")]
pub async fn oauth_callback(
    query: web::Query<OAuthCallbackQuery>
) -> impl Responder {
    // OAuth callback - exchange code for token and create session
    log::info!("OAuth callback received: code={}, state={}", query.code, query.state);
    
    // Session creation ready - JWT token would be created here
    HttpResponse::Ok().json(serde_json::json!({
        "message": "OAuth callback successful",
        "code": query.code,
        "state": query.state,
        "status": "ready_for_session",
        "next_steps": ["Exchange code for token", "Fetch user info", "Create JWT", "Set session cookie"]
    }))
}
