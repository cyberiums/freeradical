use actix_web::web;
use log::info;

use crate::models::db_connection::DatabasePool;
use crate::services::ai_providers::openai::OpenAIProvider;
use crate::services::ai_providers::anthropic::AnthropicProvider;
use crate::services::ai_providers::google::GoogleProvider;
use crate::services::ai_providers::{AIProvider, CompletionOptions};
use crate::services::ai_key_manager::{AIKeyManager, AIProviderType};
use crate::services::ai_rate_limiter::{AIRateLimiter, AICostTracker};
use crate::services::errors_service::CustomHttpError;

/// AI Command Executor
/// Executes AI commands with real provider integration
pub struct AICommandExecutor {
    key_manager: AIKeyManager,
    rate_limiter: AIRateLimiter,
    cost_tracker: AICostTracker,
}

/// Command execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub output: String,
    pub provider: String,
    pub model: String,
    pub tokens_used: (usize, usize), // (input, output)
    pub cost: f64,
    pub execution_time_ms: u128,
}

impl AICommandExecutor {
    /// Create new executor
    pub fn new() -> Self {
        info!("✅ AI Command Executor initialized");
        Self {
            key_manager: AIKeyManager::new(),
            rate_limiter: AIRateLimiter::new(),
            cost_tracker: AICostTracker::new(),
        }
    }

    /// Execute /generate-page command
    pub async fn generate_page(
        &self,
        pool: web::Data<DatabasePool>,
        topic: String,
    ) -> Result<ExecutionResult, CustomHttpError> {
        let start = std::time::Instant::now();
        info!("Executing /generate-page for topic: {}", topic);

        // Get OpenAI key
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::OpenAI, None)
            .await?;

        // Check rate limit (estimate 2500 tokens)
        self.rate_limiter
            .check_limit("openai", 2500)
            .map_err(|e| CustomHttpError::BadRequest(format!("Rate limit: {}", e)))?;

        // Create provider
        let provider = OpenAIProvider::gpt4(api_key);

        // Build prompt
        let prompt = format!(
            "Write a comprehensive, well-structured blog post about '{}'.\n\n\
            Requirements:\n\
            - Use markdown formatting\n\
            - Include an engaging title (H1)\n\
            - Write an introduction that hooks the reader\n\
            - Create 3-5 main sections with H2 headers\n\
            - Include practical examples and actionable advice\n\
            - End with a clear conclusion\n\
            - Aim for 800-1200 words\n\
            - Use a professional but accessible tone",
            topic
        );

        let options = CompletionOptions {
            temperature: 0.7,
            max_tokens: 2000,
            ..Default::default()
        };

        // Execute
        let response = provider.complete(&prompt, options).await
            .map_err(|e| CustomHttpError::InternalServerError(format!("AI error: {}", e)))?;

        // Track usage
        let input_tokens = response.usage.prompt_tokens;
        let output_tokens = response.usage.completion_tokens;
        
        self.rate_limiter
            .record_usage("openai", response.usage.total_tokens as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::OpenAI, None, response.usage.total_tokens)
            .await?;

        let cost = self.cost_tracker
            .track_cost("openai", &response.model, input_tokens, output_tokens)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        Ok(ExecutionResult {
            output: response.text,
            provider: "openai".to_string(),
            model: response.model,
            tokens_used: (input_tokens, output_tokens),
            cost,
            execution_time_ms: start.elapsed().as_millis(),
        })
    }

    /// Execute /optimize-seo command
    pub async fn optimize_seo(
        &self,
        pool: web::Data<DatabasePool>,
        page_id: i32,
        content: String,
    ) -> Result<ExecutionResult, CustomHttpError> {
        let start = std::time::Instant::now();
        info!("Executing /optimize-seo for page {}", page_id);

        // Get Anthropic key (Claude is excellent at SEO)
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::Anthropic, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("anthropic", 2000)
            .map_err(|e| CustomHttpError::BadRequest(format!("Rate limit: {}", e)))?;

        // Create provider
        let provider = AnthropicProvider::claude3_sonnet(api_key);

        // Build prompt
        let prompt = format!(
            "Analyze this content for SEO optimization:\n\n{}\n\n\
            Provide specific recommendations for:\n\
            1. Title tag optimization (50-60 characters)\n\
            2. Meta description (150-160 characters)\n\
            3. Header hierarchy (H1-H6 structure)\n\
            4. Keyword placement and density\n\
            5. Internal linking opportunities\n\
            6. Image alt text suggestions\n\
            7. Content readability improvements\n\
            8. Schema markup recommendations\n\n\
            Format as actionable checklist in markdown.",
            content
        );

        let options = CompletionOptions {
            temperature: 0.5,
            max_tokens: 1500,
            ..Default::default()
        };

        let response = provider.complete(&prompt, options).await
            .map_err(|e| CustomHttpError::InternalServerError(format!("AI error: {}", e)))?;

        // Track usage
        let input_tokens = response.usage.prompt_tokens;
        let output_tokens = response.usage.completion_tokens;
        
        self.rate_limiter
            .record_usage("anthropic", response.usage.total_tokens as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::Anthropic, None, response.usage.total_tokens)
            .await?;

        let cost = self.cost_tracker
            .track_cost("anthropic", &response.model, input_tokens, output_tokens)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        Ok(ExecutionResult {
            output: response.text,
            provider: "anthropic".to_string(),
            model: response.model,
            tokens_used: (input_tokens, output_tokens),
            cost,
            execution_time_ms: start.elapsed().as_millis(),
        })
    }

    /// Execute /summarize command
    pub async fn summarize(
        &self,
        pool: web::Data<DatabasePool>,
        url: String,
        content: String,
    ) -> Result<ExecutionResult, CustomHttpError> {
        let start = std::time::Instant::now();
        info!("Executing /summarize for URL: {}", url);

        // Use Google Gemini (fast and cost-effective)
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::Google, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("google", 1500)
            .map_err(|e| CustomHttpError::BadRequest(format!("Rate limit: {}", e)))?;

        // Create provider
        let provider = GoogleProvider::gemini_15_flash(api_key);

        // Build prompt
        let prompt = format!(
            "Summarize the following content from {}:\n\n{}\n\n\
            Provide:\n\
            1. A brief one-sentence summary\n\
            2. 3-5 key points\n\
            3. Main takeaways\n\
            4. Target audience\n\n\
            Keep it concise and actionable.",
            url, content
        );

        let options = CompletionOptions {
            temperature: 0.3,
            max_tokens: 800,
            ..Default::default()
        };

        let response = provider.complete(&prompt, options).await
            .map_err(|e| CustomHttpError::InternalServerError(format!("AI error: {}", e)))?;

        // Track usage
        let input_tokens = response.usage.prompt_tokens;
        let output_tokens = response.usage.completion_tokens;
        
        self.rate_limiter
            .record_usage("google", response.usage.total_tokens as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::Google, None, response.usage.total_tokens)
            .await?;

        let cost = self.cost_tracker
            .track_cost("google", &response.model, input_tokens, output_tokens)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        Ok(ExecutionResult {
            output: response.text,
            provider: "google".to_string(),
            model: response.model,
            tokens_used: (input_tokens, output_tokens),
            cost,
            execution_time_ms: start.elapsed().as_millis(),
        })
    }

    /// Execute /translate command
    pub async fn translate(
        &self,
        pool: web::Data<DatabasePool>,
        page_id: i32,
        content: String,
        target_language: String,
    ) -> Result<ExecutionResult, CustomHttpError> {
        let start = std::time::Instant::now();
        info!("Executing /translate page {} to {}", page_id, target_language);

        // Use OpenAI (excellent at translation)
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::OpenAI, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("openai", 2000)
            .map_err(|e| CustomHttpError::BadRequest(format!("Rate limit: {}", e)))?;

        // Create provider
        let provider = OpenAIProvider::gpt35_turbo(api_key);

        // Build prompt
        let prompt = format!(
            "Translate the following content to {} (language code: {}):\n\n{}\n\n\
            Requirements:\n\
            - Maintain the original formatting and structure\n\
            - Preserve markdown syntax\n\
            - Keep proper nouns and technical terms appropriate\n\
            - Ensure cultural relevance\n\
            - Maintain the same tone and style",
            target_language, target_language, content
        );

        let options = CompletionOptions {
            temperature: 0.3,
            max_tokens: 2000,
            ..Default::default()
        };

        let response = provider.complete(&prompt, options).await
            .map_err(|e| CustomHttpError::InternalServerError(format!("AI error: {}", e)))?;

        // Track usage
        let input_tokens = response.usage.prompt_tokens;
        let output_tokens = response.usage.completion_tokens;
        
        self.rate_limiter
            .record_usage("openai", response.usage.total_tokens as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::OpenAI, None, response.usage.total_tokens)
            .await?;

        let cost = self.cost_tracker
            .track_cost("openai", &response.model, input_tokens, output_tokens)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        Ok(ExecutionResult {
            output: response.text,
            provider: "openai".to_string(),
            model: response.model,
            tokens_used: (input_tokens, output_tokens),
            cost,
            execution_time_ms: start.elapsed().as_millis(),
        })
    }
}

impl Default for AICommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = AICommandExecutor::new();
        // Verify it initializes without panic
        assert!(true);
    }
}
