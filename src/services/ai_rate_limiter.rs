use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{info, warn, error};

/// AI Rate Limiter
/// Manages rate limits for different AI providers
pub struct AIRateLimiter {
    limits: Arc<Mutex<HashMap<String, ProviderRateLimit>>>,
}

/// Rate limit configuration per provider
#[derive(Debug, Clone)]
pub struct ProviderRateLimit {
    /// Maximum requests per minute
    pub max_requests_per_minute: u32,
    /// Maximum tokens per minute
    pub max_tokens_per_minute: u64,
    /// Current request count
    requests_this_minute: u32,
    /// Current token count
    tokens_this_minute: u64,
    /// Window start time
    window_start: Instant,
}

impl ProviderRateLimit {
    pub fn new(max_requests_per_minute: u32, max_tokens_per_minute: u64) -> Self {
        Self {
            max_requests_per_minute,
            max_tokens_per_minute,
            requests_this_minute: 0,
            tokens_this_minute: 0,
            window_start: Instant::now(),
        }
    }

    /// Reset window if minute has elapsed
    fn maybe_reset_window(&mut self) {
        if self.window_start.elapsed() >= Duration::from_secs(60) {
            self.requests_this_minute = 0;
            self.tokens_this_minute = 0;
            self.window_start = Instant::now();
        }
    }

    /// Check if request can proceed
    pub fn can_proceed(&mut self, estimated_tokens: u64) -> Result<(), String> {
        self.maybe_reset_window();

        if self.requests_this_minute >= self.max_requests_per_minute {
            return Err(format!(
                "Rate limit exceeded: {} requests/min (max: {})",
                self.requests_this_minute, self.max_requests_per_minute
            ));
        }

        if self.tokens_this_minute + estimated_tokens > self.max_tokens_per_minute {
            return Err(format!(
                "Token limit exceeded: {} tokens/min (max: {})",
                self.tokens_this_minute + estimated_tokens,
                self.max_tokens_per_minute
            ));
        }

        Ok(())
    }

    /// Record a request
    pub fn record_request(&mut self, tokens_used: u64) {
        self.maybe_reset_window();
        self.requests_this_minute += 1;
        self.tokens_this_minute += tokens_used;
    }

    /// Get current usage
    pub fn get_usage(&mut self) -> (u32, u64) {
        self.maybe_reset_window();
        (self.requests_this_minute, self.tokens_this_minute)
    }
}

impl AIRateLimiter {
    /// Create a new rate limiter with default limits
    pub fn new() -> Self {
        let mut limits = HashMap::new();

        // OpenAI limits (approximate)
        limits.insert(
            "openai".to_string(),
            ProviderRateLimit::new(10_000, 2_000_000), // 10k RPM, 2M TPM
        );

        // Anthropic limits
        limits.insert(
            "anthropic".to_string(),
            ProviderRateLimit::new(5_000, 100_000), // 5k RPM, 100k TPM
        );

        // Google Gemini limits
        limits.insert(
            "google".to_string(),
            ProviderRateLimit::new(60, 32_000), // 60 RPM, 32k TPM
        );

        info!("✅ AI Rate Limiter initialized with provider limits");

        Self {
            limits: Arc::new(Mutex::new(limits)),
        }
    }

    /// Check if request can proceed
    pub fn check_limit(
        &self,
        provider: &str,
        estimated_tokens: u64,
    ) -> Result<(), String> {
        let mut limits = self.limits.lock().map_err(|e| format!("Lock error: {}", e))?;

        let limit = limits
            .get_mut(provider)
            .ok_or_else(|| format!("Unknown provider: {}", provider))?;

        limit.can_proceed(estimated_tokens)
    }

    /// Record a completed request
    pub fn record_usage(
        &self,
        provider: &str,
        tokens_used: u64,
    ) -> Result<(), String> {
        let mut limits = self.limits.lock().map_err(|e| format!("Lock error: {}", e))?;

        let limit = limits
            .get_mut(provider)
            .ok_or_else(|| format!("Unknown provider: {}", provider))?;

        limit.record_request(tokens_used);
        Ok(())
    }

    /// Get current usage for a provider
    pub fn get_usage(&self, provider: &str) -> Result<(u32, u64), String> {
        let mut limits = self.limits.lock().map_err(|e| format!("Lock error: {}", e))?;

        let limit = limits
            .get_mut(provider)
            .ok_or_else(|| format!("Unknown provider: {}", provider))?;

        Ok(limit.get_usage())
    }

    /// Update limits for a provider
    pub fn update_limits(
        &self,
        provider: &str,
        max_requests_per_minute: u32,
        max_tokens_per_minute: u64,
    ) -> Result<(), String> {
        let mut limits = self.limits.lock().map_err(|e| format!("Lock error: {}", e))?;

        limits.insert(
            provider.to_string(),
            ProviderRateLimit::new(max_requests_per_minute, max_tokens_per_minute),
        );

        info!("Updated rate limits for {}: {} RPM, {} TPM", provider, max_requests_per_minute, max_tokens_per_minute);
        Ok(())
    }
}

impl Default for AIRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// AI Cost Tracker
/// Tracks costs per provider and model
pub struct AICostTracker {
    costs: Arc<Mutex<HashMap<String, f64>>>,
}

/// Pricing per 1M tokens (approximate, as of 2024)
pub struct ModelPricing {
    pub input_price: f64,  // Per 1M input tokens
    pub output_price: f64, // Per 1M output tokens
}

impl ModelPricing {
    /// Get pricing for a model
    pub fn get_pricing(model: &str) -> Self {
        match model {
            // OpenAI GPT-4
            "gpt-4" => ModelPricing {
                input_price: 30.0,
                output_price: 60.0,
            },
            "gpt-4-turbo" => ModelPricing {
                input_price: 10.0,
                output_price: 30.0,
            },
            
            // OpenAI GPT-3.5
            "gpt-3.5-turbo" => ModelPricing {
                input_price: 0.5,
                output_price: 1.5,
            },

            // Anthropic Claude-3
            "claude-3-opus-20240229" => ModelPricing {
                input_price: 15.0,
                output_price: 75.0,
            },
            "claude-3-sonnet-20240229" => ModelPricing {
                input_price: 3.0,
                output_price: 15.0,
            },
            "claude-3-haiku-20240307" => ModelPricing {
                input_price: 0.25,
                output_price: 1.25,
            },

            // Google Gemini
            "gemini-1.5-pro" => ModelPricing {
                input_price: 7.0,
                output_price: 21.0,
            },
            "gemini-1.5-flash" => ModelPricing {
                input_price: 0.35,
                output_price: 1.05,
            },

            // Default fallback
            _ => {
                warn!("Unknown model pricing: {}, using default", model);
                ModelPricing {
                    input_price: 10.0,
                    output_price: 30.0,
                }
            }
        }
    }

    /// Calculate cost for token usage
    pub fn calculate_cost(&self, input_tokens: usize, output_tokens: usize) -> f64 {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * self.input_price;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * self.output_price;
        input_cost + output_cost
    }
}

impl AICostTracker {
    pub fn new() -> Self {
        info!("✅ AI Cost Tracker initialized");
        Self {
            costs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Track cost for a request
    pub fn track_cost(
        &self,
        provider: &str,
        model: &str,
        input_tokens: usize,
        output_tokens: usize,
    ) -> Result<f64, String> {
        let pricing = ModelPricing::get_pricing(model);
        let cost = pricing.calculate_cost(input_tokens, output_tokens);

        let mut costs = self.costs.lock().map_err(|e| format!("Lock error: {}", e))?;
        
        let key = format!("{}:{}", provider, model);
        *costs.entry(key).or_insert(0.0) += cost;

        info!("Tracked ${:.6} for {} ({} in, {} out)", cost, model, input_tokens, output_tokens);
        Ok(cost)
    }

    /// Get total cost for a provider
    pub fn get_provider_cost(&self, provider: &str) -> Result<f64, String> {
        let costs = self.costs.lock().map_err(|e| format!("Lock error: {}", e))?;
        
        let total: f64 = costs
            .iter()
            .filter(|(key, _)| key.starts_with(&format!("{}:", provider)))
            .map(|(_, cost)| cost)
            .sum();

        Ok(total)
    }

    /// Get total cost across all providers
    pub fn get_total_cost(&self) -> Result<f64, String> {
        let costs = self.costs.lock().map_err(|e| format!("Lock error: {}", e))?;
        Ok(costs.values().sum())
    }

    /// Get detailed cost breakdown
    pub fn get_cost_breakdown(&self) -> Result<HashMap<String, f64>, String> {
        let costs = self.costs.lock().map_err(|e| format!("Lock error: {}", e))?;
        Ok(costs.clone())
    }

    /// Check if cost exceeds budget
    pub fn check_budget(&self, budget: f64) -> Result<bool, String> {
        let total = self.get_total_cost()?;
        if total >= budget {
            error!("⚠️  Budget exceeded: ${:.2} / ${:.2}", total, budget);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for AICostTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_basic() {
        let limiter = AIRateLimiter::new();
        assert!(limiter.check_limit("openai", 1000).is_ok());
    }

    #[test]
    fn test_cost_calculation() {
        let pricing = ModelPricing::get_pricing("gpt-4");
        let cost = pricing.calculate_cost(1000, 500);
        assert!(cost > 0.0);
        assert!(cost < 1.0); // Should be small for 1500 tokens
    }

    #[test]
    fn test_cost_tracker() {
        let tracker = AICostTracker::new();
        let cost = tracker.track_cost("openai", "gpt-4", 1000, 500).unwrap();
        assert!(cost > 0.0);
        
        let total = tracker.get_total_cost().unwrap();
        assert_eq!(cost, total);
    }
}
