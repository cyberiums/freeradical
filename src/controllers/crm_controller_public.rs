
// ===== PUBLIC ENDPOINTS (No Auth Required) =====

use crate::services::email_verification_service::EmailVerificationService;
use crate::services::email_service::EmailService;

#[derive(Debug, Deserialize)]
pub struct PublicCustomerRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub lifecycle_stage: Option<String>,
    pub tags: Option<Vec<String>>,
    pub source: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub tenant_id: Option<i32>,
}

/// Create customer with email verification (Public API - No Auth)
#[utoipa::path(
    post,
    path = "/v1/public/crm/customers",
    tag = "Public - CRM",
    request_body = PublicCustomerRequest,
    responses(
        (status = 200, description = "Verification email sent. Check your inbox."),
        (status = 400, description = "Invalid email or email already registered")
    )
)]
pub async fn create_customer_public(
    pool: web::Data<DbPool>,
    body: web::Json<PublicCustomerRequest>,
    email_service: web::Data<EmailService>,
) -> Result<HttpResponse, CustomHttpError> {
    // 1. Validate email format
    if !body.email.contains('@') || body.email.len() < 5 {
        return Err(CustomHttpError::BadRequest("Invalid email format".into()));
    }
    
    // 2. Check if customer already exists
    let email_check = body.email.clone();
    let tenant_check = body.tenant_id;
    let pool_check = pool.clone();
    
    let exists = web::block(move || {
        let mut conn = pool_check.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
        })?;
        
        use crate::schema::crm_customers;
        use diesel::dsl::count;
        
        let mut query = crm_customers::table
            .filter(crm_customers::email.eq(&email_check))
            .into_boxed();
        
        if let Some(tid) = tenant_check {
            query = query.filter(crm_customers::tenant_id.eq(tid));
        }
        
        let count_result: i64 = query
            .select(count(crm_customers::id))
            .first(&mut conn)
            .unwrap_or(0);
        
        Ok::<bool, CustomHttpError>(count_result > 0)
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
    
    if exists {
        return Err(CustomHttpError::BadRequest("Email already registered".into()));
    }
    
    // 3. Create pending verification with original payload
    let payload = serde_json::to_value(&*body)
        .map_err(|e| CustomHttpError::InternalServerError(format!("Serialization error: {}", e)))?;
    
    EmailVerificationService::create_and_send(
        &pool,
        "crm_customer",
        &body.email,
        payload,
        body.tenant_id,
        &email_service,
    ).await?;
    
    // 4. Return success message
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Verification email sent. Please check your inbox.",
        "expires_in_hours": 12
    })))
}

/// Verify email and complete customer creation
#[utoipa::path(
    get,
    path = "/v1/public/crm/verify/{token}",
    tag = "Public - CRM",
    params(
        ("token" = String, Path, description = "Verification token from email")
    ),
    responses(
        (status = 200, description = "Email verified successfully (HTML page)"),
        (status = 404, description = "Invalid or expired token")
    )
)]
pub async fn verify_customer_email(
    pool: web::Data<DbPool>,
    token: web::Path<String>,
) -> Result<HttpResponse, CustomHttpError> {
    // 1. Verify token and get payload
    let (email, payload, tenant_id) = EmailVerificationService::verify_and_get_payload(
        &pool,
        &token
    ).await?;
    
    // 2. Deserialize payload back to PublicCustomerRequest
    let customer_request: PublicCustomerRequest = serde_json::from_value(payload)
        .map_err(|e| CustomHttpError::InternalServerError(format!("Deserialization error: {}", e)))?;
    
    // 3. Create the actual CRM customer record
    let pool_clone = pool.clone();
    let customer = web::block(move || {
        let mut conn = pool_clone.get().map_err(|e| {
            CustomHttpError::InternalServerError(format!("DB connection error: {}", e))
        })?;
        
        use crate::schema::crm_customers;
        use crate::models::crm_models::NewCrmCustomer;
        
        let new_customer = NewCrmCustomer {
            email: customer_request.email.clone(),
            first_name: customer_request.first_name,
            last_name: customer_request.last_name,
            lifecycle_stage: customer_request.lifecycle_stage.or(Some("lead".to_string())),
            tags: customer_request.tags.map(|t| serde_json::to_value(t).ok()).flatten(),
            source: customer_request.source,
            metadata: customer_request.metadata,
            tenant_id: customer_request.tenant_id.or(tenant_id),
            health_score: Some(50), // Default
            churn_risk: Some("low".to_string()),
        };
        
        diesel::insert_into(crm_customers::table)
            .values(&new_customer)
            .execute(&mut conn)
            .map_err(|e| CustomHttpError::InternalServerError(format!("Failed to create customer: {}", e)))?;
        
        Ok::<String, CustomHttpError>(customer_request.email.clone())
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))??;
    
    // 4. Return HTML success page
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Email Verified</title>
                <style>
                    body {{
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        min-height: 100vh;
                        margin: 0;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    }}
                    .container {{
                        background: white;
                        padding: 50px;
                        border-radius: 10px;
                        box-shadow: 0 10px 40px rgba(0,0,0,0.2);
                        text-align: center;
                        max-width: 500px;
                    }}
                    h1 {{
                        color: #667eea;
                        margin-bottom: 20px;
                        font-size: 2.5rem;
                    }}
                    p {{
                        color: #666;
                        line-height: 1.6;
                        font-size: 1.1rem;
                    }}
                    .checkmark {{
                        font-size: 4rem;
                        color: #4CAF50;
                        margin-bottom: 20px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="checkmark">✅</div>
                    <h1>Email Verified!</h1>
                    <p>Thank you for verifying your email address.</p>
                    <p>You're all set! You'll start receiving updates soon.</p>
                </div>
            </body>
            </html>
        "#)))
}
