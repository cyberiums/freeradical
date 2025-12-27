use async_trait::async_trait;
use futures_util::Stream;
use log::{info, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{AIProvider, AIProviderError, CompletionOptions, CompletionResponse, TokenUsage};

/// Google Gemini provider
pub struct GoogleProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GoogleContent>,
    generation_config: GoogleGenerationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleContent {
    parts: Vec<GooglePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GoogleGenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: usize,
    #[serde(rename = "topP")]
    top_p: f32,
}

#[derive(Debug, Deserialize)]
struct GoogleResponse {
    candidates: Vec<GoogleCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: GoogleUsageMetadata,
}

#[derive(Debug, Deserialize)]
struct GoogleCandidate {
    content: GoogleContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleUsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: usize,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: usize,
    #[serde(rename = "totalTokenCount")]
    total_token_count: usize,
}

impl GoogleProvider {
    /// Create new Google Gemini provider
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let client = Client::new();
        let model = model.unwrap_or_else(|| "gemini-1.5-pro".to_string());

        info!("✅ Google Gemini provider initialized with model: {}", model);

        Self {
            client,
            api_key,
            model,
        }
    }

    /// Create with Gemini 1.5 Pro
    pub fn gemini_15_pro(api_key: String) -> Self {
        Self::new(api_key, Some("gemini-1.5-pro".to_string()))
    }

    /// Create with Gemini 1.5 Flash (faster, cheaper)
    pub fn gemini_15_flash(api_key: String) -> Self {
        Self::new(api_key, Some("gemini-1.5-flash".to_string()))
    }

    /// Get API endpoint for model
    fn get_endpoint(&self) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        )
    }
}

#[async_trait]
impl AIProvider for GoogleProvider {
    async fn complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<CompletionResponse, AIProviderError> {
        info!("Google Gemini completion request for {} tokens", options.max_tokens);

        let mut request = GoogleRequest {
            contents: vec![GoogleContent {
                parts: vec![GooglePart {
                    text: prompt.to_string(),
                }],
                role: Some("user".to_string()),
            }],
            system_instruction: None,
            generation_config: GoogleGenerationConfig {
                temperature: options.temperature,
                max_output_tokens: options.max_tokens,
                top_p: options.top_p,
            },
        };

        // Add system instruction if provided
        if let Some(system_prompt) = options.system_prompt {
            request.system_instruction = Some(GoogleContent {
                parts: vec![GooglePart {
                    text: system_prompt,
                }],
                role: None,
            });
        }

        let response = self
            .client
            .post(&self.get_endpoint())
            .header("Content-Type", "application/json")
            .query(&[("key", &self.api_key)])
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Google API network error: {}", e);
                AIProviderError::NetworkError(format!("Request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Google API error {}: {}", status, error_text);
            
            return Err(if status == 429 {
                AIProviderError::RateLimitExceeded(error_text)
            } else if status == 401 || status == 403 {
                AIProviderError::InvalidApiKey
            } else {
                AIProviderError::ApiError(format!("HTTP {}: {}", status, error_text))
            });
        }

        let google_response: GoogleResponse = response
            .json()
            .await
            .map_err(|e| {
                error!("Google parsing error: {}", e);
                AIProviderError::ParsingError(format!("Failed to parse response: {}", e))
            })?;

        let candidate = google_response
            .candidates
            .first()
            .ok_or_else(|| AIProviderError::ApiError("No candidates in response".to_string()))?;

        let text = candidate
            .content
            .parts
            .first()
            .map(|p| p.text.clone())
            .unwrap_or_default();

        let usage = TokenUsage {
            prompt_tokens: google_response.usage_metadata.prompt_token_count,
            completion_tokens: google_response.usage_metadata.candidates_token_count,
            total_tokens: google_response.usage_metadata.total_token_count,
        };

        info!("✅ Google Gemini completion: {} tokens used", usage.total_tokens);

        Ok(CompletionResponse {
            text,
            model: self.model.clone(),
            usage,
            finish_reason: candidate.finish_reason.clone().unwrap_or_default(),
            function_call: None,
        })
    }

    async fn stream_complete(
        &self,
        _prompt: &str,
        _options: CompletionOptions,
    ) -> Result<Box<dyn Stream<Item = Result<String, AIProviderError>> + Unpin + Send>, AIProviderError> {
        Err(AIProviderError::UnsupportedOperation(
            "Streaming not yet implemented for Google Gemini".to_string(),
        ))
    }

    fn count_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token
        text.len() / 4
    }

    fn name(&self) -> &str {
        "google"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn supports_functions(&self) -> bool {
        // Gemini 1.5 supports function calling
        self.model.contains("gemini-1.5")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_generation() {
        let provider = GoogleProvider::gemini_15_flash("test-key".to_string());
        let endpoint = provider.get_endpoint();
        assert!(endpoint.contains("gemini-1.5-flash"));
        assert!(endpoint.contains("generateContent"));
    }
}
