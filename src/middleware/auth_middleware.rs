use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,    // User email
    pub role: String,     // User role
    pub exp: usize,       // Expiration timestamp
}

/// User context attached to request
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: i32,
    pub email: String,
    pub role: String,
}

impl Claims {
    pub fn to_user_context(&self) -> Result<UserContext, String> {
        let user_id = self.sub.parse::<i32>()
            .map_err(|_| "Invalid user ID in token")?;
        
        Ok(UserContext {
            user_id,
            email: self.email.clone(),
            role: self.role.clone(),
        })
    }
}

/// Authentication middleware factory
pub struct AuthMiddleware {
    secret: String,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        
        Self { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            secret: self.secret.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        
        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                if h.starts_with("Bearer ") {
                    Some(h[7..].to_string())
                } else {
                    None
                }
            });
        
        // If no token, allow request to continue (route-level guards handle protection)
        if token.is_none() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        
        // Validate JWT token
        let token = token.unwrap();
        let validation = Validation::new(Algorithm::HS256);
        
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => {
                // Convert claims to user context
                match token_data.claims.to_user_context() {
                    Ok(user_context) => {
                        // Attach user context to request extensions
                        req.extensions_mut().insert(user_context);
                        
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    }
                    Err(e) => {
                        Box::pin(async move {
                            Err(actix_web::error::ErrorUnauthorized(e))
                        })
                    }
                }
            }
            Err(e) => {
                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))
                })
            }
        }
    }
}

/// Helper to create JWT token
pub fn create_jwt_token(user_id: i32, email: String, role: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        email,
        role,
        exp: expiration,
    };
    
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Helper to extract user context from request
pub fn get_user_context(req: &actix_web::HttpRequest) -> Option<UserContext> {
    req.extensions().get::<UserContext>().cloned()
}
