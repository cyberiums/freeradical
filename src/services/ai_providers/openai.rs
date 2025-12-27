use async_trait::async_trait;
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures_util::Stream;
use log::{info, error};

use super::{AIProvider, AIProviderError, CompletionOptions, CompletionResponse, TokenUsage};

/// OpenAI provider (GPT-4, GPT-3.5, etc.)
pub struct OpenAIProvider {
    client: Client<async_openai::config::OpenAIConfig>,
    model: String,
}

impl OpenAIProvider {
    /// Create new OpenAI provider
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let config = async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key);
        
        let client = Client::with_config(config);
        let model = model.unwrap_or_else(|| "gpt-4".to_string());

        info!("✅ OpenAI provider initialized with model: {}", model);

        Self { client, model }
    }

    /// Create with default GPT-4
    pub fn gpt4(api_key: String) -> Self {
        Self::new(api_key, Some("gpt-4".to_string()))
    }

    /// Create with GPT-3.5 Turbo
    pub fn gpt35_turbo(api_key: String) -> Self {
        Self::new(api_key, Some("gpt-3.5-turbo".to_string()))
    }

    /// Convert options to messages
    fn build_messages(
        &self,
        prompt: &str,
        options: &CompletionOptions,
    ) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = Vec::new();

        // Add system message if provided
        if let Some(system_prompt) = &options.system_prompt {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .unwrap()
                    .into(),
            );
        }

        // Add user message
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()
                .unwrap()
                .into(),
        );

        messages
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<CompletionResponse, AIProviderError> {
        info!("OpenAI completion request for {} tokens", options.max_tokens);

        let messages = self.build_messages(prompt, &options);

        let mut request_builder = CreateChatCompletionRequestArgs::default();
        request_builder
            .model(&self.model)
            .messages(messages)
            .temperature(options.temperature)
            .max_tokens(options.max_tokens as u16)
            .top_p(options.top_p)
            .frequency_penalty(options.frequency_penalty)
            .presence_penalty(options.presence_penalty);

        if let Some(stop) = options.stop_sequences {
            request_builder.stop(stop);
        }

        let request = request_builder
            .build()
            .map_err(|e| AIProviderError::ApiError(format!("Request build error: {}", e)))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| {
                error!("OpenAI API error: {}", e);
                AIProviderError::ApiError(format!("OpenAI error: {}", e))
            })?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| AIProviderError::ApiError("No choices in response".to_string()))?;

        let text = choice
            .message
            .content
            .clone()
            .unwrap_or_default();

        // Handle usage which is optional
        let usage_data = response.usage.as_ref().ok_or_else(|| 
            AIProviderError::ApiError("No usage data in response".to_string())
        )?;

        let usage = TokenUsage {
            prompt_tokens: usage_data.prompt_tokens as usize,
            completion_tokens: usage_data.completion_tokens as usize,
            total_tokens: usage_data.total_tokens as usize,
        };

        info!("✅ OpenAI completion: {} tokens used", usage.total_tokens);

        // Handle finish_reason
        let finish_reason = choice.finish_reason
            .as_ref()
            .map(|r| format!("{:?}", r))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(CompletionResponse {
            text,
            model: response.model,
            usage,
            finish_reason,
            function_call: None,
        })
    }

    async fn stream_complete(
        &self,
        _prompt: &str,
        _options: CompletionOptions,
    ) -> Result<Box<dyn Stream<Item = Result<String, AIProviderError>> + Unpin + Send>, AIProviderError> {
        // Streaming implementation would go here
        // For now, return unsupported
        Err(AIProviderError::UnsupportedOperation(
            "Streaming not yet implemented for OpenAI".to_string(),
        ))
    }

    fn count_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token
        // For production, use tiktoken-rs
        text.len() / 4
    }

    fn name(&self) -> &str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn supports_functions(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_counting() {
        let provider = OpenAIProvider::new("test-key".to_string(), None);
        let tokens = provider.count_tokens("Hello, world! This is a test.");
        assert!(tokens > 0);
        assert!(tokens < 20); // Rough estimate
    }
}
