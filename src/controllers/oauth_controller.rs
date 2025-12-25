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
    // Generate random state
    let state = format!("{:x}", rand::random::<u64>());
    
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
    // TODO: Exchange code for token, create user session
    HttpResponse::Ok().json(serde_json::json!({
        "message": "OAuth callback received",
        "code": query.code,
        "state": query.state
    }))
}
