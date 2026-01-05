use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::{web, HttpRequest, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use uuid::Uuid;

use crate::models::user_models::{MutUser, User, LoginRequest, Enable2faRequest};
use crate::models::{pool_handler, Model, DatabasePool};
use crate::services::totp_service::TotpService;
use crate::services::auth_service::{authenticate, encrypt, encrypt_password, Claims};
use crate::services::errors_service::CustomHttpError;
use serde_json;

/// Create a new user
#[utoipa::path(
    post,
    path = "/v1/users",
    tag = "Internal - Users",
    request_body = MutUser,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn create_user(
    new: web::Json<MutUser>,
    pool: web::Data<DatabasePool>,
    email_service: web::Data<crate::services::email_service::EmailService>,
) -> Result<HttpResponse, CustomHttpError> {
    // Clone pool before pool_handler consumes it (needed for EmailVerificationService)
    let pool_clone = pool.clone();
    let mut mysql_pool = pool_handler(pool)?;

    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);
    salted_user.uuid = Some(Uuid::new_v4().to_string());

    User::create(&salted_user, &mut mysql_pool)?;


    // Send verification email (welcome email will be sent AFTER verification)
    use crate::services::email_verification_service::EmailVerificationService;
    
    match EmailVerificationService::create_and_send(
        &pool_clone,
        "user_registration",
        &salted_user.username,
        serde_json::json!({
            "user_uuid": salted_user.uuid,
            "username": salted_user.username.clone(),
        }),
        None, // tenant_id - will be added when multi-tenancy is enabled
        &email_service,
    ).await {
        Ok(token) => log::info!("✅ Verification email sent to {} with token preview: {}...", salted_user.username, &token[..20]),
        Err(e) => log::error!("❌ Failed to send verification email to {}: {:?}", salted_user.username, e),
    }


    Ok(HttpResponse::Created().json(&new.clone()))
}

/// Get user by UUID
#[utoipa::path(
    get,
    path = "/v1/users/{uuid}",
    tag = "Internal - Users",
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let user: User = User::read_one(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(&user))
}

/// Update user
#[utoipa::path(
    put,
    path = "/v1/users/{uuid}",
    tag = "Internal - Users",
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    request_body = MutUser,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn update_user(
    id: web::Path<String>,
    new: web::Json<MutUser>,
    pool: web::Data<DatabasePool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    // TODO maybe make this only happen whenever the password changes?
    let mut salted_user = new.clone();

    // if you're trying to change someone elses data, don't allow it.
    // id param is username, check against claim.email (which is username)
    if id.clone() != claim.email {
        return Err(CustomHttpError::Unauthorized("Cannot update another user's data".to_string()));
    }

    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);

    let exp_time = chrono::Utc::now() + chrono::Duration::days(10);

    // give them a new token just in case they update their username.
    let claim = Claims {
        exp: (exp_time).timestamp() as usize,
        sub: claim.sub, // Keep same user ID
        email: salted_user.username.clone(),
        role: claim.role, // Keep same role
    };

    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hours(1);

    let token_enc = encrypt(claim)?;
    let cookie = Cookie::build("auth", token_enc.clone())
        .expires(Some(time))
        .path("/")
        .finish();

    let user = HttpResponse::Ok().cookie(cookie).json(&new.clone());
    salted_user.token = Some(token_enc);
    User::update(id.clone(), &salted_user, &mut mysql_pool)?;

    Ok(user)
}

/// Delete user
#[utoipa::path(
    delete,
    path = "/v1/users/{uuid}",
    tag = "Internal - Users",
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<DatabasePool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;

    let res = User::delete(id.clone(), &mut mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "Customer - Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful, auth cookie set"),
        (status = 401, description = "Invalid credentials or 2FA code"),
        (status = 403, description = "Default root account already initialized")
    )
)]
pub async fn login(
    user: web::Json<LoginRequest>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut mysql_pool = pool_handler(pool)?;
    let arg = Argon2::default();

    let read_user = User::read_one(user.username.clone(), &mut mysql_pool)?;

    let is_default = read_user.username == "root" && read_user.password == "";

    // if you're trying to login to a root user more than once with no password set, send back a forbidden.
    if read_user.token.is_some() && is_default {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // default password handler.
    if is_default {
        let mut new_user = MutUser {
            uuid: None,
            username: user.username.clone(),
            password: None,
            token: None,
            two_factor_secret: None,
            two_factor_enabled: None,
        };
        
        let cookie = login_res(&read_user)?; // Pass actual user with ID
        let cookie_response = HttpResponse::Accepted().cookie(cookie.clone()).finish();

        new_user.token = Some(cookie.value().to_string());
        User::update_with_token(&new_user, &mut mysql_pool)?;

        return Ok(cookie_response);
    }
    
    let read_user_password = PasswordHash::new(&read_user.password).unwrap();

    match arg.verify_password(
        user.password.as_bytes(),
        &read_user_password,
    ) {
        Ok(_) => {
            // 2FA Verification
            if read_user.two_factor_enabled {
                match &user.two_factor_code {
                    Some(code) => {
                         if let Some(secret) = &read_user.two_factor_secret {
                             if !TotpService::verify(secret, code).unwrap_or(false) {
                                  return Ok(HttpResponse::Unauthorized().json("Invalid 2FA code"));
                             }
                         }
                    }
                    None => return Ok(HttpResponse::Unauthorized().json("2FA code required")),
                }
            }
            
            let mut new_user = MutUser {
                uuid: None,
                username: user.username.clone(),
                password: None,
                token: None,
                two_factor_secret: None,
                two_factor_enabled: None,
            };

            let cookie = login_res(&read_user)?; // Pass actual user with ID
            let cookie_response = HttpResponse::Ok().cookie(cookie.clone()).finish();

            new_user.token = Some(cookie.value().to_string());
            User::update_with_token(&new_user, &mut mysql_pool)?;

            Ok(cookie_response)
        }
        _ => Ok(HttpResponse::Unauthorized().json("Failed to authenticate.")),
    }
}

fn login_res(user: &User) -> Result<Cookie<'_>, CustomHttpError> {
    let claim = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize,
        sub: user.id.to_string(),
        email: user.username.clone(),
        role: "user".to_string(), // Default role
    };
    
    let token_enc = encrypt(claim)?;

    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hours(1);
    let cookie = Cookie::build("auth", token_enc.clone())
        .expires(Some(time))
        .path("/")
        .finish();

    Ok(cookie)
}

/// Logout user
#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "Customer - Authentication",
    responses(
        (status = 200, description = "Logged out successfully")
    )
)]
pub async fn logout() -> Result<HttpResponse, CustomHttpError> {
    let cookie = Cookie::build("auth", "")
        .expires(Some(OffsetDateTime::now_utc()))
        .path("/")
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

/// Check if user is logged in
#[utoipa::path(
    get,
    path = "/v1/auth/status",
    tag = "Customer - Authentication",
    responses(
        (status = 200, description = "Authenticated", body = User),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn check_login(
    req: HttpRequest,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, CustomHttpError> {
    let auth_header = req.headers().get("authorization");

    let auth_str = match auth_header {
        Some(h) => h.to_str().unwrap_or("").to_string(),
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    if auth_str.is_empty() {
         return Ok(HttpResponse::Unauthorized().finish());
    }

    let auth_res = authenticate(auth_str, &pool).await;

    match auth_res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}

/// Setup 2FA for user
#[utoipa::path(
    post,
    path = "/v1/auth/2fa/setup",
    tag = "Customer - Authentication",
    responses(
        (status = 200, description = "2FA setup initiated"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn setup_2fa(
    path: web::Path<String>,
    _: web::Data<DatabasePool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    if path.clone() != claim.sub {
         return Err(CustomHttpError::Unauthorized("Unauthorized access".to_string()));
    }
    
    let (secret, qr) = TotpService::generate_secret(&path).map_err(|e| {
         log::error!("2FA Gen Error: {}", e);
         CustomHttpError::BadRequest("2FA generation failed".to_string())
    })?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "secret": secret,
        "qr": qr
    })))
}

/// Enable 2FA for user
#[utoipa::path(
    post,
    path = "/v1/auth/2fa/enable",
    tag = "Customer - Authentication",
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    request_body = Enable2faRequest,
    responses(
        (status = 200, description = "2FA enabled successfully"),
        (status = 400, description = "Invalid token"),
        (status = 401, description = "Not authenticated")
    ),
    security((
        "bearer_auth" = []
    ))
)]
pub async fn enable_2fa(
    path: web::Path<String>,
    body: web::Json<Enable2faRequest>,
    pool: web::Data<DatabasePool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    if path.clone() != claim.sub {
        return Err(CustomHttpError::Unauthorized("Unauthorized access".to_string()));
    }
    
    if !TotpService::verify(&body.secret, &body.code).unwrap_or(false) {
         return Ok(HttpResponse::BadRequest().json("Invalid Code"));
    }
    
    let mut mysql_pool = pool_handler(pool)?;
    
    let update_user = MutUser {
         uuid: None,
         username: path.clone(),
         password: None,
         token: None,
         two_factor_secret: Some(body.secret.clone()),
         two_factor_enabled: Some(true),
    };
    
    User::update(path.clone(), &update_user, &mut mysql_pool)?;
    
    Ok(HttpResponse::Ok().json("2FA Enabled"))
}
