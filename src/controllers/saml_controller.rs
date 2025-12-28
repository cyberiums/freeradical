use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SamlConfig {
    pub entity_id: String,
    pub sso_url: String,
    pub x509_cert: String,
}

pub async fn metadata() -> impl Responder {
    // Return SP Metadata XML
    // In production, generating this dynamically based on the tenant
    let xml = r#"<EntityDescriptor entityID="https://oxidly.com/saml/metadata" xmlns="urn:oasis:names:tc:SAML:2.0:metadata">
    <SPSSODescriptor protocolSupportEnumeration="urn:oasis:names:tc:SAML:2.0:protocol">
        <NameIDFormat>urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress</NameIDFormat>
        <AssertionConsumerService index="1" Binding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST" Location="https://oxidly.com/saml/acs"/>
    </SPSSODescriptor>
</EntityDescriptor>"#;

    HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml)
}

pub async fn login(tenant_id: web::Path<String>) -> impl Responder {
    // 1. Lookup tenant SSO config from DB (stubbed for now)
    // 2. Generate SAMLRequest
    // 3. Redirect to IdP
    
    // Stub redirection
    let idp_url = format!("https://mock-idp.com/sso?tenant={}", tenant_id);
    HttpResponse::Found()
        .append_header(("Location", idp_url))
        .finish()
}

pub async fn acs(form: web::Form<std::collections::HashMap<String, String>>) -> impl Responder {
    // 1. Receive SAMLResponse
    // 2. Validate Signature using stored Cert
    // 3. Extract User Email/NameID
    // 4. Issue Session/JWT
    
    // For now, assume success and return a mock token
    println!("Received SAML Response: {:?}", form);

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "SAML Authentication Successful (Stub)",
        "token": "mock_jwt_token_123"
    }))
}

#[derive(Deserialize)]
pub struct UpdateSsoRequest {
    pub sso_url: String,
    pub entity_id: String,
    pub x509_cert: String,
}

pub async fn update_config(
    tenant_id: web::Path<i32>,
    params: web::Json<UpdateSsoRequest>
) -> impl Responder {
    // In a real implementation:
    // Diesel insert/update into tenant_sso_configs where tenant_id = tenant_id
    
    // Stub logs
    println!("Updating SSO for Tenant {}: URL={}, Entity={}", tenant_id, params.sso_url, params.entity_id);

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "SSO Configuration Updated"
    }))
}
