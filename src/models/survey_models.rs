use crate::schema::{surveys, survey_questions, survey_responses};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = surveys)]
pub struct Survey {
    pub id: i32,
    pub tenant_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = surveys)]
pub struct NewSurvey {
    pub tenant_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = survey_questions)]
pub struct Question {
    pub id: i32,
    pub survey_id: i32,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<serde_json::Value>,
    pub order_index: Option<i32>,
    pub is_required: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = survey_questions)]
pub struct NewQuestion {
    pub survey_id: i32,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<serde_json::Value>,
    pub order_index: Option<i32>,
    pub is_required: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = survey_responses)]
pub struct Response {
    pub id: i32,
    pub survey_id: i32,
    pub respondent_id: Option<i32>,
    pub answers: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = survey_responses)]
pub struct NewResponse {
    pub survey_id: i32,
    pub respondent_id: Option<i32>,
    pub answers: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
}
