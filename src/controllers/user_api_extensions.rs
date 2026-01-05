use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::{pool_handler, DatabasePool, Model};
use crate::models::user_models::User;
use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

/// User invite request
#[derive(Deserialize, ToSchema)]
pub struct InviteUserRequest {
    pub email: String,
    pub role: String,
    pub mcp_access: bool,
    pub message: Option<String>,
}

/// Invite a new user to the team
#[utoipa::path(
    post,
    path = "/v1/users/invite",
    tag = "Internal - Users",
    request_body = InviteUserRequest,
    responses(
        (status = 200, description = "Invitation sent successfully"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn invite_user(
    body: web::Json<InviteUserRequest>,
    pool: web::Data<DatabasePool>,
    email_service: web::Data<crate::services::email_service::EmailService>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    // Only admins and editors can invite users
    if claim.role != "admin" && claim.role != "editor" {
        return Err(CustomHttpError::Unauthorized("Only admins and editors can invite users".to_string()));
    }

    // Validate email format
    if !body.email.contains('@') {
        return Err(CustomHttpError::BadRequest("Invalid email address".to_string()));
    }

    // Validate role
    let valid_roles = vec!["admin", "editor", "author", "contributor", "subscriber"];
    if !valid_roles.contains(&body.role.as_str()) {
        return Err(CustomHttpError::BadRequest("Invalid role specified".to_string()));
    }

    // Check if user already exists
    let mut mysql_pool = pool_handler(pool.clone())?;
    let existing_user = User::read_one(body.email.clone(), &mut mysql_pool);
    if existing_user.is_ok() {
        return Err(CustomHttpError::BadRequest("User already exists".to_string()));
    }

    // Generate invitation token (valid for 7 days)
    use uuid::Uuid;
    let invitation_token = Uuid::new_v4().to_string();
    let expiry = chrono::Utc::now() + chrono::Duration::days(7);

    // TODO: Store invitation in database (pending_invitations table)
    // For now, we'll just send the email

    // Prepare email template variables
    let template_vars = serde_json::json!({
        "recipient_email": body.email,
        "inviter_name": claim.email,
        "role": body.role,
        "invitation_url": format!("http://localhost:5005/signup?invite={}", invitation_token),
        "personal_message": body.message.clone().unwrap_or_default(),
        "expires_at": expiry.format("%Y-%m-%d %H:%M UTC").to_string(),
    });

    // Send invitation email
    match email_service.send_email(
        &body.email,
        "You're Invited to Join Oxidly!",
        &format!(
            "You've been invited to join the Oxidly team by {}.\n\nRole: {}\n\n{}\n\nClick here to accept: http://localhost:5005/signup?invite={}\n\nThis invitation expires in 7 days.",
            claim.email,
            body.role,
            body.message.clone().unwrap_or_default(),
            invitation_token
        ),
    ).await {
        Ok(_) => {
            log::info!("✅ Invitation sent to {} by {}", body.email, claim.email);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": format!("Invitation sent to {}", body.email),
                "invitation_token": invitation_token
            })))
        },
        Err(e) => {
            log::error!("❌ Failed to send invitation to {}: {:?}", body.email, e);
            Err(CustomHttpError::InternalServerError(format!("Failed to send invitation: {}", e)))
        }
    }
}

/// Export users as CSV
#[utoipa::path(
    get,
    path = "/v1/users/export",
    tag = "Internal - Users",
    responses(
        (status = 200, description = "CSV file download", content_type = "text/csv"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn export_users(
    pool: web::Data<DatabasePool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    // Only admins can export user data
    if claim.role != "admin" {
        return Err(CustomHttpError::Unauthorized("Only admins can export user data".to_string()));
    }

    let mut mysql_pool = pool_handler(pool)?;
    
    // Get all users
    let users: Vec<User> = User::read_all(&mut mysql_pool)?;

    // Build CSV content
    let mut csv_content = String::from("UUID,Username,Role,2FA Enabled,Created At\n");
    
    for user in users {
        csv_content.push_str(&format!(
            "{},{},{},{},{}\n",
            user.uuid,
            user.username,
            "user", // Default role, could be enhanced with actual role from user_roles table
            if user.two_factor_enabled { "Yes" } else { "No" },
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S") // Would use actual created_at if available
        ));
    }

    log::info!("📥 User export requested by {}", claim.email);

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .insert_header(("Content-Disposition", "attachment; filename=\"users.csv\""))
        .body(csv_content))
}

/// Test send email template
#[derive(Deserialize, ToSchema)]
pub struct TestEmailRequest {
    pub template_key: String,
    pub test_email: String,
}

/// Send test email using a template
#[utoipa::path(
    post,
    path = "/v1/email-templates/test",
    tag = "Internal - Email Templates",
    request_body = TestEmailRequest,
    responses(
        (status = 200, description = "Test email sent successfully"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn test_email_template(
    body: web::Json<TestEmailRequest>,
    email_service: web::Data<crate::services::email_service::EmailService>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    // Only admins and editors can test email templates
    if claim.role != "admin" && claim.role != "editor" {
        return Err(CustomHttpError::Unauthorized("Only admins and editors can test email templates".to_string()));
    }

    // Validate email format
    if !body.test_email.contains('@') {
        return Err(CustomHttpError::BadRequest("Invalid email address".to_string()));
    }

    // Prepare sample template variables
    let template_vars = serde_json::json!({
        "app_name": "Oxidly",
        "username": "Test User",
        "user_email": body.test_email.clone(),
        "verification_url": "http://localhost:5005/verify?token=test-token",
        "reset_url": "http://localhost:5005/reset-password?token=test-token",
        "login_url": "http://localhost:5005/login",
        "support_email": "support@oxidly.com",
        "current_year": chrono::Utc::now().format("%Y").to_string(),
    });

    // Send test email
    match email_service.send_email(
        &body.test_email,
        &format!("Test Email: {}", body.template_key),
        &format!(
            "🧪 This is a test email for template: {}\n\nSent by: {}\n\nTemplate variables:\n{}",
            body.template_key,
            claim.email,
            serde_json::to_string_pretty(&template_vars).unwrap_or_default()
        ),
    ).await {
        Ok(_) => {
            log::info!("✅ Test email sent to {} for template {} by {}", body.test_email, body.template_key, claim.email);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": format!("Test email sent to {}", body.test_email)
            })))
        },
        Err(e) => {
            log::error!("❌ Failed to send test email: {:?}", e);
            Err(CustomHttpError::InternalServerError(format!("Failed to send test email: {}", e)))
        }
    }
}
