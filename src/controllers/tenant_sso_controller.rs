use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use crate::models::{DatabasePool, user_models};
use crate::models::tenant_sso_models::{TenantSsoConfig, MutTenantSsoConfig};
use crate::models::tenant_models::{Tenant, TenantMember};

#[derive(Serialize, Deserialize)]
pub struct UpdateSsoRequest {
    pub provider_type: String, // 'saml' or 'oidc'
    // SAML Fields
    pub entity_id: Option<String>,
    pub sso_url: Option<String>,
    pub x509_cert: Option<String>,
    // OIDC Fields
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub discovery_url: Option<String>,
    pub is_enabled: bool,
}

pub async fn update_config(
    tenant_id: web::Path<i32>,
    params: web::Json<UpdateSsoRequest>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection");
    
    // Encrypt client secret if present (Placeholder: using plaintext for now, should use encryption_service)
    let secret = params.client_secret.clone(); 

    let new_config = MutTenantSsoConfig {
        tenant_id: *tenant_id,
        provider_type: Some(params.provider_type.clone()),
        idp_entity_id: params.entity_id.clone(),
        idp_sso_url: params.sso_url.clone(),
        x509_certificate: params.x509_cert.clone(),
        client_id: params.client_id.clone(),
        client_secret: secret,
        discovery_url: params.discovery_url.clone(),
        is_enabled: Some(params.is_enabled),
    };

    match TenantSsoConfig::upsert(&new_config, &mut conn) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "SSO Configuration Updated"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

pub async fn login(
    tenant_id: web::Path<i32>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection");
    
    // 1. Lookup Tenant Config
    let config = match TenantSsoConfig::find_by_tenant(*tenant_id, &mut conn) {
        Ok(Some(c)) => c,
        Ok(None) => return HttpResponse::NotFound().body("SSO not configured for this tenant"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    if !config.is_enabled {
        return HttpResponse::BadRequest().body("SSO is disabled for this tenant");
    }

    // 2. Dispatch based on type
    match config.provider_type.as_str() {
        "oidc" => {
            // Build OIDC Authorization URL
            if let (Some(client_id), Some(discovery_url)) = (config.client_id, config.discovery_url) {
                // In production: Fetch discovery doc to get auth_endpoint
                // For now, assume a standard pattern or we need discovery logic.
                // Simplified: use discovery_url as base or assume it IS the auth url if user provided it as such
                // Typically discovery_url is .well-known/openid-configuration. 
                // Let's assume the user provided the AUTH URL for simplicity in this version, or we implement discovery fetch.
                // We will assume `discovery_url` field currently holds the AUTH URL for MVP.
                let redirect_uri = "http://localhost:8000/v1/sso/oidc/callback";
                let state = format!("tenant_{}", tenant_id); // Encode tenant_id in state
                
                let url = format!(
                    "{}?client_id={}&redirect_uri={}&scope=openid%20email%20profile&response_type=code&state={}",
                    discovery_url, client_id, redirect_uri, state
                );
                
                HttpResponse::Found().append_header(("Location", url)).finish()
            } else {
                HttpResponse::InternalServerError().body("Invalid OIDC Configuration")
            }
        },
        "saml" => {
            // Stub SAML
            if let Some(sso_url) = config.idp_sso_url {
                 HttpResponse::Found().append_header(("Location", sso_url)).finish()
            } else {
                 HttpResponse::InternalServerError().body("Invalid SAML Configuration")
            }
        },
        _ => HttpResponse::BadRequest().body("Unknown provider type")
    }
}

#[derive(Deserialize)]
pub struct OidcCallbackQuery {
    code: String,
    state: String,
}

pub async fn oidc_callback(
    query: web::Query<OidcCallbackQuery>,
    pool: web::Data<DatabasePool>
) -> impl Responder {
    // 1. Extract tenant_id from state
    let tenant_id_str = query.state.strip_prefix("tenant_").unwrap_or("0");
    let tenant_id: i32 = tenant_id_str.parse().unwrap_or(0);
    
    if tenant_id == 0 {
        return HttpResponse::BadRequest().body("Invalid State Parameter");
    }

    let mut conn = pool.get().expect("couldn't get db connection");
    
    // 2. Lookup Config to get Client Secret
    let config = match TenantSsoConfig::find_by_tenant(tenant_id, &mut conn) {
        Ok(Some(c)) => c,
        _ => return HttpResponse::BadRequest().body("Tenant Config Not Found"),
    };

    // 3. Exchange Code (Simplified: skipping actual HTTP call for brevity, but this is where it goes)
    // let client = reqwest::Client::new();
    // let token_res = client.post(...)
    
    // Mock Success
    println!("SSO Login Successful for Tenant {}", tenant_id);
    
    // 4. Create Session / JWT
    // Redirect to Tenant Dashboard
    HttpResponse::Found()
        .append_header(("Location", "/admin/dashboard"))
        .finish()
}

pub async fn saml_acs() -> impl Responder {
     HttpResponse::Ok().body("SAML ACS Endpoint (Stub)")
}

pub async fn metadata() -> impl Responder {
    HttpResponse::Ok().content_type("application/xml").body("<EntityDescriptor>...</EntityDescriptor>")
}
