use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::{db_connection, survey_models::{Survey, NewSurvey, Question, NewQuestion, Response, NewResponse}};
use crate::middleware::auth_middleware::{get_user_context, UserContext};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
pub struct CreateSurveyRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AddQuestionRequest {
    pub question_text: String,
    pub question_type: String,
    pub options: Option<serde_json::Value>,
    pub order_index: Option<i32>,
    pub is_required: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct SurveyDetails {
    pub survey: Survey,
    pub questions: Vec<Question>,
}

/// List user's surveys
#[utoipa::path(
    get,
    path = "/v1/surveys",
    tag = "Internal - System",
    responses(
        (status = 200, description = "List of surveys created by user", body = Vec<Survey>),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_surveys(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    // Need to filter by tenant_id. Assuming user_ctx has key or we look up membership?
    // For simplicity, let's assume we pass tenant_id as query param or infer from user context if possible.
    // Ideally, we list surveys for the authenticated user's active tenant or all tenants they belong to.
    // But `freeradical` context doesn't strictly enforce "current tenant" in session yet.
    // I'll list all surveys where the user is the creator OR (better) join with tenants.
    // Let's require a tenant_id query param for now, or just list all created by user.
    // The `list_my_tenants` implementation implies users can have multiple.
    
    // Let's list by creator for now.
    
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::surveys;

    let results = surveys::table
        .filter(surveys::created_by.eq(user_ctx.user_id))
        .load::<Survey>(&mut conn);

    match results {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error listing surveys: {}", e)),
    }
}

/// Create a new survey
#[utoipa::path(
    post,
    path = "/v1/surveys",
    tag = "Internal - System",
    request_body = CreateSurveyRequest,
    responses(
        (status = 200, description = "Survey created", body = Survey),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_survey(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    item: web::Json<CreateSurveyRequest>
) -> impl Responder {
    let user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };

    // We assume tenant_id is 1 for now or passed in. 
    // TODO: Pass tenant_id in request or header. default to None or 1?
    // I'll just default to None for personal surveys or explicitly require it.
    // Let's assume personal checks for now.

    let new_survey = NewSurvey {
        tenant_id: None, // Simplified for v3.0 MVP
        title: item.title.clone(),
        description: item.description.clone(),
        status: item.status.clone().or(Some("draft".to_string())),
        created_by: Some(user_ctx.user_id),
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::surveys;

    let res = diesel::insert_into(surveys::table)
        .values(&new_survey)
        .get_result::<Survey>(&mut conn);

    match res {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating survey: {}", e)),
    }
}

/// Get survey with questions
#[utoipa::path(
    get,
    path = "/v1/surveys/{id}",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Survey ID", example = 10)
    ),
    responses(
        (status = 200, description = "Survey details with questions", body = SurveyDetails),
        (status = 404, description = "Survey not found")
    )
)]
pub async fn get_survey(
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    let survey_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::{surveys, survey_questions};

    let survey_res = surveys::table
        .find(survey_id)
        .first::<Survey>(&mut conn);

    let survey = match survey_res {
        Ok(s) => s,
        Err(_) => return HttpResponse::NotFound().json("Survey not found"),
    };

    let questions_res = survey_questions::table
        .filter(survey_questions::survey_id.eq(survey_id))
        .order(survey_questions::order_index.asc())
        .load::<Question>(&mut conn)
        .unwrap_or(vec![]);

    HttpResponse::Ok().json(SurveyDetails {
        survey,
        questions: questions_res
    })
}

/// Add question to survey
#[utoipa::path(
    post,
    path = "/v1/surveys/{id}/questions",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Survey ID", example = 10)
    ),
    request_body = AddQuestionRequest,
    responses(
        (status = 200, description = "Question added", body = Question),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_question(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>,
    item: web::Json<AddQuestionRequest>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let survey_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::survey_questions;

    let new_q = NewQuestion {
        survey_id,
        question_text: item.question_text.clone(),
        question_type: item.question_type.clone(),
        options: item.options.clone(),
        order_index: item.order_index,
        is_required: item.is_required,
    };

    let res = diesel::insert_into(survey_questions::table)
        .values(&new_q)
        .get_result::<Question>(&mut conn);

    match res {
        Ok(q) => HttpResponse::Ok().json(q),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error adding question: {}", e)),
    }
}

/// Submit survey response
#[utoipa::path(
    post,
    path = "/v1/surveys/{id}/responses",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Survey ID", example = 10)
    ),
    request_body(content = String, description = "JSON answers object"),
    responses(
        (status = 200, description = "Response submitted", body = Response),
        (status = 500, description = "Error submitting response")
    )
)]
pub async fn submit_response(
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>,
    item: web::Json<serde_json::Value>
) -> impl Responder {
    let survey_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::survey_responses;

    let new_resp = NewResponse {
        survey_id,
        respondent_id: None, // Anonymous for now
        answers: item.clone(),
        metadata: None,
    };

    let res = diesel::insert_into(survey_responses::table)
        .values(&new_resp)
        .get_result::<Response>(&mut conn);

    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error submitting response: {}", e)),
    }
}

/// Get survey results
#[utoipa::path(
    get,
    path = "/v1/surveys/{id}/results",
    tag = "Internal - System",
    params(
        ("id" = i32, Path, description = "Survey ID", example = 10)
    ),
    responses(
        (status = 200, description = "Survey responses", body = Vec<Response>),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_results(
    req: HttpRequest,
    pool: web::Data<db_connection::DatabasePool>,
    path: web::Path<i32>
) -> impl Responder {
    let _user_ctx = match get_user_context(&req) {
        Some(ctx) => ctx,
        None => return HttpResponse::Unauthorized().json("User not authenticated"),
    };
    
    let survey_id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    use crate::schema::survey_responses;

    let res = survey_responses::table
        .filter(survey_responses::survey_id.eq(survey_id))
        .load::<Response>(&mut conn);

    match res {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error getting results: {}", e)),
    }
}
