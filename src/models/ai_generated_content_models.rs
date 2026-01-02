use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

use crate::schema::ai_generated_content;

/// AI Generated Content - tracks AI-generated content with quality scores
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = ai_generated_content)]
pub struct AIGeneratedContent {
    pub id: i64,
    pub page_uuid: Option<String>,
    pub content_type: String,
    pub prompt_used: Option<String>,
    pub generated_text: String,
    pub model_name: Option<String>,
    pub provider_type: Option<String>,
    pub tokens_used: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub quality_score: Option<BigDecimal>,
    pub was_accepted: Option<bool>,
    pub generated_by: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// New AI Generated Content - for inserting
#[derive(Debug, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = ai_generated_content)]
pub struct NewAIGeneratedContent {
    pub page_uuid: Option<String>,
    pub content_type: String,
    pub prompt_used: Option<String>,
    pub generated_text: String,
    pub model_name: Option<String>,
    pub provider_type: Option<String>,
    pub tokens_used: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub quality_score: Option<BigDecimal>,
    pub was_accepted: Option<bool>,
    pub generated_by: Option<i32>,
}

/// Public DTO - for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGeneratedContentDTO {
    pub id: i64,
    pub page_uuid: Option<String>,
    pub content_type: String,
    pub generated_text: String,
    pub model_name: Option<String>,
    pub provider_type: Option<String>,
    pub quality_score: Option<f64>,
    pub was_accepted: bool,
    pub created_at: Option<String>,
}

impl From<AIGeneratedContent> for AIGeneratedContentDTO {
    fn from(content: AIGeneratedContent) -> Self {
        AIGeneratedContentDTO {
            id: content.id,
            page_uuid: content.page_uuid,
            content_type: content.content_type,
            generated_text: content.generated_text,
            model_name: content.model_name,
            provider_type: content.provider_type,
            quality_score: content.quality_score.and_then(|q| q.to_string().parse().ok()),
            was_accepted: content.was_accepted.unwrap_or(false),
            created_at: content.created_at.map(|dt| dt.to_string()),
        }
    }
}
