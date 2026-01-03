use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

use super::auth_service::CryptoError;

#[derive(Debug)]
pub enum CustomHttpError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    TooManyRequests(String),
    InternalServerError(String),
    Unknown,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for CustomHttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            CustomHttpError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    code: StatusCode::BAD_REQUEST.as_u16(),
                    error: "Bad Request".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    error: "Unauthorized".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(ErrorResponse {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    error: "Forbidden".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    error: "Not Found".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::TooManyRequests(msg) => {
                HttpResponse::TooManyRequests().json(ErrorResponse {
                    code: StatusCode::TOO_MANY_REQUESTS.as_u16(),
                    error: "Too Many Requests".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    error: "Internal Server Error".to_string(),
                    message: msg.clone(),
                })
            }
            CustomHttpError::Unknown => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    error: "Internal Server Error".to_string(),
                    message: "An unknown error occurred".to_string(),
                })
            }
        }
    }
}

impl fmt::Display for CustomHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomHttpError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            CustomHttpError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            CustomHttpError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            CustomHttpError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            CustomHttpError::TooManyRequests(msg) => write!(f, "Too Many Requests: {}", msg),
            CustomHttpError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            CustomHttpError::Unknown => write!(f, "An unknown error occurred"),
        }
    }
}

impl From<diesel::result::Error> for CustomHttpError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => CustomHttpError::NotFound("Resource not found".to_string()),
            _ => CustomHttpError::InternalServerError(format!("Database error: {}", e)),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for CustomHttpError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        CustomHttpError::Unauthorized(format!("JWT error: {}", e))
    }
}

impl From<CryptoError> for CustomHttpError {
    fn from(_e: CryptoError) -> Self {
        CustomHttpError::Unauthorized("Authentication failed".to_string())
    }
}

impl From<String> for CustomHttpError {
    fn from(err: String) -> Self {
        CustomHttpError::InternalServerError(err)
    }
}
