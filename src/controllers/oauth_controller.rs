use actix_web::{HttpResponse, Responder};
use crate::services::oauth_service::OAuthService;

pub async fn google_login() -> impl Responder {
    // Generate OAuth URL using the Service layer
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:8000/oauth/callback".to_string());
    
    // Redirect URI must match the one used in exchange_code_for_token in oauth_callback_controller.rs
    match OAuthService::get_authorization_url(
        "google", 
        &redirect_uri, 
        "state_token_placeholder" // In prod, generate random state and store in cookie/session
    ) {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error generating login URL: {}", e)),
    }
}
