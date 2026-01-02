use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::DbPool;
use crate::services::errors_service::CustomHttpError;

/// Submission status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmissionStatus {
    pub id: i32,
    pub resource_type: String, // "plugin" or "theme"
    pub name: String,
    pub status: String, // "pending", "approved", "rejected"
    pub submitted_at: String,
}

/// Submit plugin request
#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmitPluginRequest {
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub version: String,
}

/// Submit plugin
#[utoipa::path(
    post,
    path = "/v1/marketplace/plugins/submit",
    tag = "Marketplace - Submissions",
    request_body = SubmitPluginRequest,
    responses(
        (status = 201, description = "Plugin submitted for review"),
        (status = 400, description = "Invalid submission")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn submit_plugin(
    _pool: web::Data<DbPool>,
    _payload: web::Json<SubmitPluginRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement plugin submission
    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": 1,
        "status": "pending",
        "message": "Plugin submitted for review (placeholder)"
    })))
}

/// List my submissions
#[utoipa::path(
    get,
    path = "/v1/marketplace/submissions",
    tag = "Marketplace - Submissions",
    responses(
        (status = 200, description = "List of submissions", body = Vec<SubmissionStatus>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_my_submissions(
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement listing user's submissions
    Ok(HttpResponse::Ok().json(Vec::<SubmissionStatus>::new()))
}

/// Get submission status
#[utoipa::path(
    get,
    path = "/v1/marketplace/submissions/{id}",
    tag = "Marketplace - Submissions",
    params(
        ("id" = i32, Path, description = "Submission ID")
    ),
    responses(
        (status = 200, description = "Submission details", body = SubmissionStatus),
        (status = 404, description = "Submission not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_submission_status(
    _pool: web::Data<DbPool>,
    submission_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement get submission
    Ok(HttpResponse::Ok().json(SubmissionStatus {
        id: submission_id.into_inner(),
        resource_type: "plugin".to_string(),
        name: "Placeholder Plugin".to_string(),
        status: "pending".to_string(),
        submitted_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// Update submission
#[utoipa::path(
    put,
    path = "/v1/marketplace/submissions/{id}",
    tag = "Marketplace - Submissions",
    params(
        ("id" = i32, Path, description = "Submission ID")
    ),
    request_body = SubmitPluginRequest,
    responses(
        (status = 200, description = "Submission updated"),
        (status = 404, description = "Submission not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_submission(
    _pool: web::Data<DbPool>,
    submission_id: web::Path<i32>,
    _payload: web::Json<SubmitPluginRequest>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement update submission
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": submission_id.into_inner(),
        "message": "Submission updated (placeholder)"
    })))
}

/// Review submission (admin only)
#[utoipa::path(
    post,
    path = "/v1/marketplace/submissions/{id}/review",
    tag = "Marketplace - Submissions",
    params(
        ("id" = i32, Path, description = "Submission ID")
    ),
    responses(
        (status = 200, description = "Review recorded"),
        (status = 403, description = "Forbidden - admin only"),
        (status = 404, description = "Submission not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn review_submission(
    _pool: web::Data<DbPool>,
    submission_id: web::Path<i32>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement review submission
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": submission_id.into_inner(),
        "message": "Review recorded (placeholder)"
    })))
}
