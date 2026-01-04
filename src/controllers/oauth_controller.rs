use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::services::oauth_service::OAuthService;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Deserialize)]
pub struct OAuthQuery {
    pub tenant_id: Option<i32>,
}

/// Initiate Google OAuth login flow
#[utoipa::path(
    get,
    path = "/v1/auth/google",
    tag = "Customer - Authentication",
    params(
        ("tenant_id" = Option<i32>, Query, description = "Tenant ID for redirect lookup")
    ),
    responses(
        (status = 302, description = "Redirect to Google OAuth consent page"),
        (status = 500, description = "Error generating OAuth URL")
    )
)]
pub async fn google_login(query: web::Query<OAuthQuery>) -> impl Responder {
    // Get tenant_id from query parameter, default to 1 for backward compatibility
    let tenant_id = query.tenant_id.unwrap_or(1);
    
    // Generate nonce for security
    let nonce = Uuid::new_v4().to_string();
    
    // Create state object with tenant_id and nonce
    let state_data = serde_json::json!({
        "tenant_id": tenant_id,
        "nonce": nonce
    });
    
    // Encode state as base64
    let state_json = state_data.to_string();
    let state = BASE64.encode(state_json.as_bytes());
    
    // Generate OAuth URL using the Service layer
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:8000/v1/auth/google/callback".to_string());
    
    match OAuthService::get_authorization_url(
        "google", 
        &redirect_uri, 
        &state  // Pass encoded state with tenant_id
    ) {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error generating login URL: {}", e)),
    }
}
