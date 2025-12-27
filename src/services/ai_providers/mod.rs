use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generic AI Provider trait
/// All AI providers (OpenAI, Anthropic, Google, etc.) implement this trait
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Generate text completion from prompt
    async fn complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<CompletionResponse, AIProviderError>;

    /// Stream text completion (for real-time responses)
    async fn stream_complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<Box<dyn futures_util::Stream<Item = Result<String, AIProviderError>> + Unpin + Send>, AIProviderError>;

    /// Count tokens in text (for cost estimation)
    fn count_tokens(&self, text: &str) -> usize;

    /// Get provider name
    fn name(&self) -> &str;

    /// Get model name
    fn model(&self) -> &str;

    /// Check if provider supports function calling
    fn supports_functions(&self) -> bool {
        false
    }
}

/// Completion options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub temperature: f32,
    pub max_tokens: usize,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

impl Default for CompletionOptions {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: None,
            system_prompt: None,
        }
    }
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub text: String,
    pub model: String,
    pub usage: TokenUsage,
    pub finish_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<serde_json::Value>,
}

/// Token usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// AI Provider errors
#[derive(Debug, thiserror::Error)]
pub enum AIProviderError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Parsing error: {0}")]
    ParsingError(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// Provider registry for managing multiple AI providers
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn AIProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, provider: Box<dyn AIProvider>) {
        self.providers.insert(name, provider);
    }

    pub fn get(&self, name: &str) -> Option<&dyn AIProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    pub fn list(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}

// Re-export provider implementations
pub mod openai;
pub mod anthropic;
pub mod google;
