use actix_web::web;
use log::info;

use crate::models::db_connection::DatabasePool;
use crate::services::command_parser::{CommandIntent, ParsedCommand};
use crate::services::ai_key_manager::{AIKeyManager, AIProviderType};
use crate::services::ai_rate_limiter::AIRateLimiter;
use crate::services::errors_service::CustomHttpError;

/// Command Router
/// Routes parsed commands to appropriate handlers
pub struct CommandRouter {
    key_manager: AIKeyManager,
    rate_limiter: AIRateLimiter,
}

/// Command execution result
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub metadata: CommandMetadata,
}

/// Command execution metadata
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    pub command_type: String,
    pub provider_used: Option<String>,
    pub tokens_used: usize,
    pub cost_estimate: f64,
    pub execution_time_ms: u128,
}

impl CommandRouter {
    /// Create a new command router
    pub fn new() -> Self {
        info!("✅ Command Router initialized");
        Self {
            key_manager: AIKeyManager::new(),
            rate_limiter: AIRateLimiter::new(),
        }
    }

    /// Route and execute a command
    pub async fn route(
        &self,
        pool: web::Data<DatabasePool>,
        command: ParsedCommand,
    ) -> Result<CommandResult, CustomHttpError> {
        let start = std::time::Instant::now();
        
        info!("Routing command: {:?}", command.intent);

        let result = match command.intent {
            CommandIntent::GeneratePage { topic } => {
                self.handle_generate_page(pool, topic).await?
            }
            CommandIntent::OptimizeSEO { page_id } => {
                self.handle_optimize_seo(pool, page_id).await?
            }
            CommandIntent::Summarize { url } => {
                self.handle_summarize(pool, url).await?
            }
            CommandIntent::Translate { page_id, language } => {
                self.handle_translate(pool, page_id, language).await?
            }
            CommandIntent::AnalyzeContent { content } => {
                self.handle_analyze(pool, content).await?
            }
            CommandIntent::Unknown => {
                return Err(CustomHttpError::BadRequest(
                    "Unknown command intent".to_string()
                ));
            }
        };

        // Add execution time
        let mut final_result = result;
        final_result.metadata.execution_time_ms = start.elapsed().as_millis();

        info!(
            "Command executed in {}ms, tokens: {}, cost: ${:.6}",
            final_result.metadata.execution_time_ms,
            final_result.metadata.tokens_used,
            final_result.metadata.cost_estimate
        );

        Ok(final_result)
    }

    /// Handle page generation command
    async fn handle_generate_page(
        &self,
        pool: web::Data<DatabasePool>,
        topic: String,
    ) -> Result<CommandResult, CustomHttpError> {
        info!("Generating page about: {}", topic);

        // Get API key for OpenAI
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::OpenAI, None)
            .await?;

        // Check rate limit (estimate ~2000 tokens)
        self.rate_limiter
            .check_limit("openai", 2000)
            .map_err(|e| CustomHttpError::BadRequest(e))?;

        // Create prompt
        let prompt = format!(
            "Generate a comprehensive blog post about '{}'. \
            Include an engaging introduction, main content with examples, \
            and a clear conclusion. Format in markdown.",
            topic
        );

        // TODO: Actually call AI provider here
        // For now, return mock response
        let output = format!("# {}\n\nThis is a generated article about {}...", topic, topic);

        // Record usage
        let tokens_used = 1500; // Mock value
        self.rate_limiter
            .record_usage("openai", tokens_used as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        // Track in database
        self.key_manager
            .track_usage(pool, AIProviderType::OpenAI, None, tokens_used)
            .await?;

        Ok(CommandResult {
            success: true,
            output,
            metadata: CommandMetadata {
                command_type: "generate_page".to_string(),
                provider_used: Some("openai".to_string()),
                tokens_used,
                cost_estimate: 0.045, // Mock cost
                execution_time_ms: 0,
            },
        })
    }

    /// Handle SEO optimization command
    async fn handle_optimize_seo(
        &self,
        pool: web::Data<DatabasePool>,
        page_id: i32,
    ) -> Result<CommandResult, CustomHttpError> {
        info!("Optimizing SEO for page {}", page_id);

        // Get API key for Anthropic (Claude is good at SEO)
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::Anthropic, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("anthropic", 1500)
            .map_err(|e| CustomHttpError::BadRequest(e))?;

        // TODO: Fetch page content and analyze
        let output = format!(
            "SEO Analysis for page #{}:\n\
            - Title: Optimize for target keywords\n\
            - Meta description: Add compelling description\n\
            - Headers: Use H1-H6 hierarchy\n\
            - Internal links: Add 3-5 relevant links\n\
            - Image alt text: Add descriptive alt attributes",
            page_id
        );

        let tokens_used = 1200;
        self.rate_limiter
            .record_usage("anthropic", tokens_used as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::Anthropic, None, tokens_used)
            .await?;

        Ok(CommandResult {
            success: true,
            output,
            metadata: CommandMetadata {
                command_type: "optimize_seo".to_string(),
                provider_used: Some("anthropic".to_string()),
                tokens_used,
                cost_estimate: 0.036,
                execution_time_ms: 0,
            },
        })
    }

    /// Handle summarize command
    async fn handle_summarize(
        &self,
        pool: web::Data<DatabasePool>,
        url: String,
    ) -> Result<CommandResult, CustomHttpError> {
        info!("Summarizing content from: {}", url);

        // Use Google Gemini (fast and cheap for summaries)
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::Google, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("google", 1000)
            .map_err(|e| CustomHttpError::BadRequest(e))?;

        // TODO: Fetch URL content and summarize
        let output = format!(
            "Summary of {}:\n\n\
            Key points:\n\
            1. Main topic overview\n\
            2. Important details\n\
            3. Conclusion and takeaways",
            url
        );

        let tokens_used = 800;
        self.rate_limiter
            .record_usage("google", tokens_used as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::Google, None, tokens_used)
            .await?;

        Ok(CommandResult {
            success: true,
            output,
            metadata: CommandMetadata {
                command_type: "summarize".to_string(),
                provider_used: Some("google".to_string()),
                tokens_used,
                cost_estimate: 0.008,
                execution_time_ms: 0,
            },
        })
    }

    /// Handle translate command
    async fn handle_translate(
        &self,
        pool: web::Data<DatabasePool>,
        page_id: i32,
        language: String,
    ) -> Result<CommandResult, CustomHttpError> {
        info!("Translating page {} to {}", page_id, language);

        // OpenAI is good at translation
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::OpenAI, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("openai", 2000)
            .map_err(|e| CustomHttpError::BadRequest(e))?;

        // TODO: Fetch page and translate
        let output = format!(
            "Translated page #{} to {}:\n\n[Translated content would appear here]",
            page_id, language
        );

        let tokens_used = 1800;
        self.rate_limiter
            .record_usage("openai", tokens_used as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::OpenAI, None, tokens_used)
            .await?;

        Ok(CommandResult {
            success: true,
            output,
            metadata: CommandMetadata {
                command_type: "translate".to_string(),
                provider_used: Some("openai".to_string()),
                tokens_used,
                cost_estimate: 0.054,
                execution_time_ms: 0,
            },
        })
    }

    /// Handle content analysis command
    async fn handle_analyze(
        &self,
        pool: web::Data<DatabasePool>,
        content: String,
    ) -> Result<CommandResult, CustomHttpError> {
        info!("Analyzing content ({} chars)", content.len());

        // Use Anthropic Claude for analysis
        let api_key = self.key_manager
            .get_key(pool.clone(), AIProviderType::Anthropic, None)
            .await?;

        // Check rate limit
        self.rate_limiter
            .check_limit("anthropic", 1000)
            .map_err(|e| CustomHttpError::BadRequest(e))?;

        let output = "Content Analysis:\n\
            - Tone: Professional\n\
            - Readability: High\n\
            - SEO potential: Good\n\
            - Suggestions: Add more examples".to_string();

        let tokens_used = 900;
        self.rate_limiter
            .record_usage("anthropic", tokens_used as u64)
            .map_err(|e| CustomHttpError::InternalServerError(e))?;

        self.key_manager
            .track_usage(pool, AIProviderType::Anthropic, None, tokens_used)
            .await?;

        Ok(CommandResult {
            success: true,
            output,
            metadata: CommandMetadata {
                command_type: "analyze_content".to_string(),
                provider_used: Some("anthropic".to_string()),
                tokens_used,
                cost_estimate: 0.027,
                execution_time_ms: 0,
            },
        })
    }
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = CommandRouter::new();
        // Just verify it doesn't panic
        assert!(true);
    }
}
