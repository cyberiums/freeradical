use actix_web::http::header::HeaderValue;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use argon2::{Argon2, PasswordHasher};
use password_hash::{SaltString, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use futures::{future::LocalBoxFuture, Future};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::errors_service::CustomHttpError;
use crate::models::{pool_handler, user_models, Model, DatabasePool};

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("An unknown cryptographic error has occured")]
    Unknown,
    #[error("User has failed their token comparison")]
    FailedComparison,
    #[error("There is no user present")]
    NoUser,
    #[error("The user is not logged in")]
    NotLoggedIn,
    #[error("No auth header present.")]
    NoAuthHeader,
    #[error("Password operation failed.")]
    OperationFail
}

impl From<jsonwebtoken::errors::Error> for CryptoError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            _ => Self::Unknown,
        }
    }
}

impl From<argon2::password_hash::Error> for CryptoError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => Self::OperationFail
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,    // User email (username in DB)
    pub role: String,     // User role
    pub exp: usize,       // Expiration timestamp
}

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

pub fn encrypt(claim: Claims) -> Result<String, CryptoError> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        
    let encoded_token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(encoded_token)
}

pub fn decrypt(jwt: &String) -> Result<Claims, CryptoError> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let decoded_token = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(decoded_token.claims)
}

pub fn compare(
    token: &Claims,
    enc_token: &String,
    pool: &DatabasePool,
) -> Result<(), CryptoError> {
    let mut pool_conn = pool.get().map_err(|_| CryptoError::Unknown)?; 
    // Lookup by email (which is username in DB)
    if let Ok(user) = user_models::User::read_one(token.email.clone(), &mut pool_conn) {
        if user.token.is_none() {
            return Err(CryptoError::NotLoggedIn);
        }
        // verify against the encrypted version of the token.
        if user.token == Some(enc_token.clone()) {
            return Ok(());
        } else {
            return Err(CryptoError::FailedComparison);
        };
    } else {
        return Err(CryptoError::NoUser);
    }
}

pub fn encrypt_password(password: &String) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

impl FromRequest for Claims {
    type Error = CustomHttpError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<DatabasePool>>().unwrap().to_owned();
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_val) = auth_header {
             // Convert to owned string to satisfy static lifetime
             let auth_str = match std::str::from_utf8(auth_val.as_bytes()) {
                 Ok(s) => s.to_string(),
                 Err(_) => return Box::pin(async { Err(CryptoError::NoAuthHeader.into()) }),
             };
             
             let pool = pool.as_ref().clone(); // Clone the pool (Arc internally)
             
             return Box::pin(async move {
                 authenticate(auth_str, &pool).await
             });
        }
        
        Box::pin(async { Err(CryptoError::NoAuthHeader.into()) })
    }
}

pub fn authenticate(
    auth_token_str: String,
    pool: &DatabasePool,
) -> impl Future<Output = Result<Claims, CustomHttpError>> {
    let encrypted_token = if auth_token_str.starts_with("Bearer ") {
        auth_token_str[7..].to_string()
    } else {
        auth_token_str
    };
    
    // We need to move pool into the async block, so clone it
    let pool = pool.clone();

    async move {
        // Run synchronous operations
        let decrypted_token = decrypt(&encrypted_token);

        let mut logged_in = Err(CryptoError::NotLoggedIn);
        if let Ok(decrypted_token) = &decrypted_token {
            // compare is synchronous but uses pool
             logged_in = compare(&decrypted_token, &encrypted_token, &pool);
        }

        match logged_in {
            Ok(_) => Ok(decrypted_token?),
            Err(e) => Err(e.into()),
        }
    }
}

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
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
