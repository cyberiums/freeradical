use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::models::{DbPool, crm_models::*};
use crate::services::{crm_service, errors_service::CustomHttpError};

// ===== Request DTOs =====

#[derive(Debug, Deserialize)]
pub struct CustomerFilters {
    pub lifecycle_stage: Option<String>,
    pub min_health_score: Option<i32>,
    pub churn_risk: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub user_id: i32,
    pub lifecycle_stage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub lifecycle_stage: Option<String>,
    pub health_score: Option<i32>,
    pub churn_risk: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInteractionRequest {
    pub customer_id: i32,
    pub interaction_type: String,
    pub interaction_channel: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub outcome: Option<String>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub description: Option<String>,
    pub criteria: serde_json::Value,
    pub is_dynamic: Option<bool>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub campaign_type: String,
    pub segment_id: Option<i32>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub scheduled_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub customer_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub task_type: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub assigned_to: Option<i32>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub customer_id: i32,
    pub note_text: String,
    pub is_pinned: Option<bool>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TaskFilters {
    pub customer_id: Option<i32>,
    pub assigned_to: Option<i32>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

// ===== Response DTOs =====

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize)]
pub struct CustomerDetailResponse {
    pub customer: CrmCustomer,
    pub recent_interactions: Vec<CrmInteraction>,
    pub active_tasks: Vec<CrmTask>,
}

// ===== Customer Endpoints =====

/// List customers with optional filters
pub async fn list_customers(
    query: web::Query<CustomerFilters>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::list_customers(
        pool,
        query.lifecycle_stage.clone(),
        query.min_health_score,
    ).await
}

/// Get customer profile by ID
pub async fn get_customer_profile(
    customer_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_customer_profile(pool, *customer_id).await
}

/// Create or get customer from user_id
pub async fn create_customer(
    request: web::Json<CreateCustomerRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_or_create_customer(pool, request.user_id).await
        .map(|customer| HttpResponse::Created().json(customer))
}

/// Update customer information
pub async fn update_customer(
    customer_id: web::Path<i32>,
    request: web::Json<UpdateCustomerRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement update_customer in service
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Update customer endpoint - to be implemented"
    })))
}

/// Delete customer (soft delete)
pub async fn delete_customer(
    customer_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    // TODO: Implement soft delete
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Delete customer endpoint - to be implemented"
    })))
}

// ===== Interaction Endpoints =====

/// Get customer interaction timeline
pub async fn get_customer_timeline(
    customer_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_customer_timeline(pool, *customer_id).await
}

/// Create new customer interaction
pub async fn create_interaction(
    request: web::Json<CreateInteractionRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_interaction = NewCrmInteraction {
        customer_id: request.customer_id,
        interaction_type: request.interaction_type.clone(),
        interaction_channel: request.interaction_channel.clone(),
        subject: request.subject.clone(),
        description: request.description.clone(),
        outcome: request.outcome.clone(),
        created_by: request.created_by,
    };
    
    crm_service::create_interaction(pool, new_interaction).await
}

/// Get interaction details
pub async fn get_interaction(
    interaction_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_interaction_details(pool, *interaction_id).await
}

// ===== Segment Endpoints =====

/// List all segments
pub async fn list_segments(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::list_segments(pool).await
}

/// Create new segment
pub async fn create_segment(
    request: web::Json<CreateSegmentRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_segment = NewCrmSegment {
        name: request.name.clone(),
        description: request.description.clone(),
        criteria: request.criteria.clone(),
        is_dynamic: request.is_dynamic,
        created_by: request.created_by,
    };
    
    crm_service::create_segment(pool, new_segment).await
}

/// Get segment members
pub async fn get_segment_members(
    segment_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_segment_members(pool, *segment_id).await
}

// ===== Campaign Endpoints =====

/// Create new campaign
pub async fn create_campaign(
    request: web::Json<CreateCampaignRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_campaign = NewCrmCampaign {
        name: request.name.clone(),
        campaign_type: request.campaign_type.clone(),
        status: Some("draft".to_string()),
        segment_id: request.segment_id,
        subject: request.subject.clone(),
        content: request.content.clone(),
        scheduled_at: request.scheduled_at,
        created_by: request.created_by,
    };
    
    crm_service::create_campaign(pool, new_campaign).await
}

/// List campaigns
pub async fn list_campaigns(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::list_campaigns(pool).await
}

// ===== Task Endpoints =====

/// Create new task
pub async fn create_task(
    request: web::Json<CreateTaskRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_task = NewCrmTask {
        customer_id: request.customer_id,
        title: request.title.clone(),
        description: request.description.clone(),
        task_type: request.task_type.clone(),
        priority: request.priority.clone(),
        due_date: request.due_date,
        assigned_to: request.assigned_to,
        created_by: request.created_by,
    };
    
    crm_service::create_task(pool, new_task).await
}

/// List tasks with filters
pub async fn list_tasks(
    query: web::Query<TaskFilters>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::list_tasks_filtered(
        pool,
        query.customer_id,
        query.assigned_to,
        query.status.clone(),
    ).await
}

/// Update task
pub async fn update_task(
    task_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    // For now, just update status to "completed"
    crm_service::update_task_status(
        pool,
        *task_id,
        "completed".to_string(),
        Some(chrono::Utc::now().naive_utc()),
    ).await
}

// ===== Note Endpoints =====

/// Add note to customer
pub async fn add_customer_note(
    customer_id: web::Path<i32>,
    request: web::Json<CreateNoteRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let new_note = NewCrmNote {
        customer_id: *customer_id,
        note_text: request.note_text.clone(),
        is_pinned: request.is_pinned,
        created_by: request.created_by,
    };
    
    crm_service::add_customer_note(pool, new_note).await
}

/// Get customer notes
pub async fn get_customer_notes(
    customer_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, CustomHttpError> {
    crm_service::get_notes_for_customer(pool, *customer_id).await
}
