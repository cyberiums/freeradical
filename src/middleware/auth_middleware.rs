use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, 
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::future::{ready, Ready};

// Import Claims and UserContext from auth_service
pub use crate::services::auth_service::{Claims, UserContext};

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
        
        // Extract token from Authorization header or cookie
        let mut token_str = None;
        
        if let Some(h) = req.headers().get("Authorization") {
             if let Ok(s) = h.to_str() {
                 if s.starts_with("Bearer ") {
                     token_str = Some(s[7..].to_string());
                 }
             }
        }
        
        // Also check cookies if no header
        if token_str.is_none() {
             if let Some(cookie) = req.cookie("auth") {
                 token_str = Some(cookie.value().to_string());
             }
        }
        
        // If no token, allow request to continue (route-level guards handle protection)
        if token_str.is_none() {
            // debug print removed to reduce noise in production, can actuate if needed
            // println!("AuthMiddleware: No token found. Proceeding.");
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        
        // Validate JWT token
        let token = token_str.unwrap();
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
                         // If we cannot parse context (e.g. invalid permissions), assume unauthorized
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

/// Helper to extract user context from request
pub fn get_user_context(req: &actix_web::HttpRequest) -> Option<UserContext> {
    req.extensions().get::<UserContext>().cloned()
}
