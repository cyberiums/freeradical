use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// // use crate::schema::{ai_provider_configs, ai_usage_log, ai_generated_content, ai_generation_queue};
 // Temporarily disabled

/// AI Provider Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    GCP,      // Google Cloud Platform Vertex AI
    Azure,    // Azure OpenAI Service
    Custom,   // Self-hosted or other providers
}

/// AI Provider Configuration
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = ai_provider_configs)]
pub struct AIProviderConfig {
    pub id: i64,
    pub provider_type: String,
    pub name: String,
    pub api_key_encrypted: Option<Vec<u8>>,
    pub config: serde_json::Value,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub priority: Option<i32>,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
    pub tokens_used_today: Option<i32>,
    pub tokens_used_month: Option<i32>,
    pub cost_month_cents: Option<i32>,
    pub last_used_at: Option<NaiveDateTime>,
    pub last_reset_daily: Option<NaiveDateTime>,
    pub last_reset_monthly: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<i32>,
}

/// New AI Provider for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = ai_provider_configs)]
pub struct NewAIProviderConfig {
    pub provider_type: String,
    pub name: String,
    pub api_key_encrypted: Option<Vec<u8>>,
    pub config: serde_json::Value,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub priority: Option<i32>,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
    pub created_by: Option<i32>,
}

/// AI Usage Log Entry
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = ai_usage_log)]
pub struct AIUsageLog {
    pub id: i64,
    pub provider_id: Option<i64>,
    pub user_id: Option<i32>,
    pub operation: Option<String>,
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
    pub cost_cents: Option<i32>,
    pub model: Option<String>,
    pub latency_ms: Option<i32>,
    pub success: Option<bool>,
    pub error: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// New AI Usage Log for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = ai_usage_log)]
pub struct NewAIUsageLog {
    pub provider_id: Option<i64>,
    pub user_id: Option<i32>,
    pub operation: String,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub cost_cents: i32,
    pub model: String,
    pub latency_ms: i32,
    pub success: bool,
    pub error: Option<String>,
}

/// AI Generated Content
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = ai_generated_content)]
pub struct AIGeneratedContent {
    pub id: i64,
    pub page_id: Option<i64>,
    pub provider_id: Option<i64>,
    pub content_type: Option<String>,
    pub prompt: Option<String>,
    pub generated_content: Option<String>,
    pub model: Option<String>,
    pub tokens_used: Option<i32>,
    pub approved: Option<bool>,
    pub approved_by: Option<i32>,
    pub approved_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

/// AI Generation Queue Task
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = ai_generation_queue)]
pub struct AIGenerationTask {
    pub id: i64,
    pub page_id: Option<i64>,
    pub task_type: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
    pub provider_id: Option<i64>,
    pub input_data: Option<serde_json::Value>,
    pub result_data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub attempts: Option<i32>,
    pub max_attempts: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}

/// Provider Config for API responses (without encrypted key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfigPublic {
    pub id: i64,
    pub provider_type: String,
    pub name: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub is_default: bool,
    pub priority: i32,
    pub daily_token_limit: Option<i32>,
    pub monthly_budget_cents: Option<i32>,
    pub tokens_used_today: i32,
    pub tokens_used_month: i32,
    pub cost_month_cents: i32,
    pub has_api_key: bool,
    pub created_at: NaiveDateTime,
}

impl From<AIProviderConfig> for AIProviderConfigPublic {
    fn from(config: AIProviderConfig) -> Self {
        Self {
            id: config.id,
            provider_type: config.provider_type,
            name: config.name,
            config: config.config,
            is_active: config.is_active.unwrap_or(true),
            is_default: config.is_default.unwrap_or(false),
            priority: config.priority.unwrap_or(100),
            daily_token_limit: config.daily_token_limit,
            monthly_budget_cents: config.monthly_budget_cents,
            tokens_used_today: config.tokens_used_today.unwrap_or(0),
            tokens_used_month: config.tokens_used_month.unwrap_or(0),
            cost_month_cents: config.cost_month_cents.unwrap_or(0),
            has_api_key: config.api_key_encrypted.is_some(),
            created_at: config.created_at.unwrap_or_else(|| chrono::Utc::now().naive_utc()),
        }
    }
}
