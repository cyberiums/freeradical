use async_trait::async_trait;
use futures_util::Stream;
use log::{info, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{AIProvider, AIProviderError, CompletionOptions, CompletionResponse, TokenUsage};

/// Anthropic provider (Claude-3, Claude-2, etc.)
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: usize,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    #[serde(rename = "type")]
    response_type: String,
    role: String,
    content: Vec<AnthropicContent>,
    model: String,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: usize,
    output_tokens: usize,
}

impl AnthropicProvider {
    /// Create new Anthropic provider
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let client = Client::new();
        let model = model.unwrap_or_else(|| "claude-3-opus-20240229".to_string());

        info!("✅ Anthropic provider initialized with model: {}", model);

        Self {
            client,
            api_key,
            model,
        }
    }

    /// Create with Claude-3 Opus
    pub fn claude3_opus(api_key: String) -> Self {
        Self::new(api_key, Some("claude-3-opus-20240229".to_string()))
    }

    /// Create with Claude-3 Sonnet
    pub fn claude3_sonnet(api_key: String) -> Self {
        Self::new(api_key, Some("claude-3-sonnet-20240229".to_string()))
    }

    /// Create with Claude-3 Haiku (fastest, cheapest)
    pub fn claude3_haiku(api_key: String) -> Self {
        Self::new(api_key, Some("claude-3-haiku-20240307".to_string()))
    }
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    async fn complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<CompletionResponse, AIProviderError> {
        info!("Anthropic completion request for {} tokens", options.max_tokens);

        let request = AnthropicRequest {
            model: self.model.clone(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: options.max_tokens,
            temperature: options.temperature,
            system: options.system_prompt,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Anthropic network error: {}", e);
                AIProviderError::NetworkError(format!("Request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Anthropic API error {}: {}", status, error_text);
            
            return Err(if status == 429 {
                AIProviderError::RateLimitExceeded(error_text)
            } else if status == 401 {
                AIProviderError::InvalidApiKey
            } else {
                AIProviderError::ApiError(format!("HTTP {}: {}", status, error_text))
            });
        }

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| {
                error!("Anthropic parsing error: {}", e);
                AIProviderError::ParsingError(format!("Failed to parse response: {}", e))
            })?;

        let text = anthropic_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let usage = TokenUsage {
            prompt_tokens: anthropic_response.usage.input_tokens,
            completion_tokens: anthropic_response.usage.output_tokens,
            total_tokens: anthropic_response.usage.input_tokens + anthropic_response.usage.output_tokens,
        };

        info!("✅ Anthropic completion: {} tokens used", usage.total_tokens);

        Ok(CompletionResponse {
            text,
            model: anthropic_response.model,
            usage,
            finish_reason: anthropic_response.stop_reason.unwrap_or_default(),
            function_call: None,
        })
    }

    async fn stream_complete(
        &self,
        _prompt: &str,
        _options: CompletionOptions,
    ) -> Result<Box<dyn Stream<Item = Result<String, AIProviderError>> + Unpin + Send>, AIProviderError> {
        Err(AIProviderError::UnsupportedOperation(
            "Streaming not yet implemented for Anthropic".to_string(),
        ))
    }

    fn count_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token for Claude
        text.len() / 4
    }

    fn name(&self) -> &str {
        "anthropic"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn supports_functions(&self) -> bool {
        // Claude-3 supports tool use
        self.model.contains("claude-3")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = AnthropicProvider::claude3_haiku("test-key".to_string());
        assert_eq!(provider.name(), "anthropic");
        assert!(provider.model().contains("haiku"));
    }
}
