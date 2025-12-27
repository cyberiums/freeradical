use log::{error, warn, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Error Recovery System
/// Handles AI operation errors with logging and recovery strategies
pub struct ErrorRecoverySystem {
    error_log: Arc<Mutex<Vec<ErrorRecord>>>,
    recovery_strategies: HashMap<ErrorType, RecoveryStrategy>,
}

/// Error record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord {
    pub id: String,
    pub error_type: ErrorType,
    pub error_message: String,
    pub context: ErrorContext,
    pub timestamp: chrono::NaiveDateTime,
    pub severity: ErrorSeverity,
    pub resolved: bool,
    pub recovery_attempted: bool,
    pub recovery_success: bool,
}

/// Error types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorType {
    RateLimitExceeded,
    APIKeyInvalid,
    NetworkTimeout,
    InvalidResponse,
    InsufficientCredits,
    ModelUnavailable,
    ContentPolicyViolation,
    Unknown,
}

/// Error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub operation: String,
    pub provider: String,
    pub model: Option<String>,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
}

/// Error severity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,      // Minor, can retry
    Medium,   // Important, needs attention
    High,     // Critical, immediate action
    Critical, // System-level failure
}

/// Recovery strategy
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    Retry { max_attempts: u32, backoff_seconds: u64 },
    SwitchProvider { fallback_provider: String },
    UseCache,
    Fail,
}

impl ErrorRecoverySystem {
    /// Create new error recovery system
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        
        // Define default recovery strategies
        strategies.insert(
            ErrorType::RateLimitExceeded,
            RecoveryStrategy::Retry { max_attempts: 3, backoff_seconds: 60 },
        );
        strategies.insert(
            ErrorType::NetworkTimeout,
            RecoveryStrategy::Retry { max_attempts: 2, backoff_seconds: 5 },
        );
        strategies.insert(
            ErrorType::APIKeyInvalid,
            RecoveryStrategy::Fail,
        );
        strategies.insert(
            ErrorType::ModelUnavailable,
            RecoveryStrategy::SwitchProvider { fallback_provider: "openai".to_string() },
        );

        info!("✅ Error Recovery System initialized with {} strategies", strategies.len());

        Self {
            error_log: Arc::new(Mutex::new(Vec::new())),
            recovery_strategies: strategies,
        }
    }

    /// Log an error
    pub fn log_error(
        &self,
        error_type: ErrorType,
        error_message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    ) -> String {
        let error_id = uuid::Uuid::new_v4().to_string();
        
        let record = ErrorRecord {
            id: error_id.clone(),
            error_type: error_type.clone(),
            error_message: error_message.clone(),
            context: context.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            severity: severity.clone(),
            resolved: false,
            recovery_attempted: false,
            recovery_success: false,
        };

        let mut log = self.error_log.lock().unwrap();
        log.push(record);

        // Log based on severity
        match severity {
            ErrorSeverity::Critical => {
                error!("[CRITICAL] {} - {}: {} (Operation: {}, Provider: {})",
                    error_id, error_type.as_str(), error_message, context.operation, context.provider);
            }
            ErrorSeverity::High => {
                error!("[HIGH] {} - {}: {} (Operation: {}, Provider: {})",
                    error_id, error_type.as_str(), error_message, context.operation, context.provider);
            }
            ErrorSeverity::Medium => {
                warn!("[MEDIUM] {} - {}: {} (Operation: {}, Provider: {})",
                    error_id, error_type.as_str(), error_message, context.operation, context.provider);
            }
            ErrorSeverity::Low => {
                info!("[LOW] {} - {}: {} (Operation: {}, Provider: {})",
                    error_id, error_type.as_str(), error_message, context.operation, context.provider);
            }
        }

        error_id
    }

    /// Get recovery strategy for error type
    pub fn get_recovery_strategy(&self, error_type: &ErrorType) -> Option<RecoveryStrategy> {
        self.recovery_strategies.get(error_type).cloned()
    }

    /// Mark error as recovery attempted
    pub fn mark_recovery_attempted(&self, error_id: &str, success: bool) {
        let mut log = self.error_log.lock().unwrap();
        
        if let Some(record) = log.iter_mut().find(|r| r.id == error_id) {
            record.recovery_attempted = true;
            record.recovery_success = success;
            
            if success {
                record.resolved = true;
                info!("Error {} recovered successfully", error_id);
            } else {
                warn!("Error {} recovery failed", error_id);
            }
        }
    }

    /// Get error statistics
    pub fn get_statistics(&self) -> ErrorStatistics {
        let log = self.error_log.lock().unwrap();
        
        let total_errors = log.len();
        let unresolved = log.iter().filter(|r| !r.resolved).count();
        let recovered = log.iter().filter(|r| r.recovery_success).count();
        
        // Count by type
        let mut by_type: HashMap<String, usize> = HashMap::new();
        for record in log.iter() {
            *by_type.entry(record.error_type.as_str().to_string()).or_insert(0) += 1;
        }

        // Count by severity
        let critical = log.iter().filter(|r| r.severity == ErrorSeverity::Critical).count();
        let high = log.iter().filter(|r| r.severity == ErrorSeverity::High).count();
        let medium = log.iter().filter(|r| r.severity == ErrorSeverity::Medium).count();
        let low = log.iter().filter(|r| r.severity == ErrorSeverity::Low).count();

        ErrorStatistics {
            total_errors,
            unresolved,
            recovered,
            by_type,
            by_severity: SeverityBreakdown {
                critical,
                high,
                medium,
                low,
            },
        }
    }

    /// Get recent errors
    pub fn get_recent_errors(&self, limit: usize) -> Vec<ErrorRecord> {
        let log = self.error_log.lock().unwrap();
        log.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear resolved errors
    pub fn clear_resolved(&self) -> usize {
        let mut log = self.error_log.lock().unwrap();
        let original_len = log.len();
        log.retain(|r| !r.resolved);
        let cleared = original_len - log.len();
        
        if cleared > 0 {
            info!("Cleared {} resolved errors", cleared);
        }
        
        cleared
    }
}

/// Error statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStatistics {
    pub total_errors: usize,
    pub unresolved: usize,
    pub recovered: usize,
    pub by_type: HashMap<String, usize>,
    pub by_severity: SeverityBreakdown,
}

/// Severity breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityBreakdown {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorType::RateLimitExceeded => "rate_limit_exceeded",
            ErrorType::APIKeyInvalid => "api_key_invalid",
            ErrorType::NetworkTimeout => "network_timeout",
            ErrorType::InvalidResponse => "invalid_response",
            ErrorType::InsufficientCredits => "insufficient_credits",
            ErrorType::ModelUnavailable => "model_unavailable",
            ErrorType::ContentPolicyViolation => "content_policy_violation",
            ErrorType::Unknown => "unknown",
        }
    }
}

impl Default for ErrorRecoverySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_error() {
        let system = ErrorRecoverySystem::new();
        
        let context = ErrorContext {
            operation: "generate_content".to_string(),
            provider: "openai".to_string(),
            model: Some("gpt-4".to_string()),
            user_id: None,
            request_id: Some("req-123".to_string()),
        };

        let error_id = system.log_error(
            ErrorType::RateLimitExceeded,
            "Rate limit exceeded".to_string(),
            context,
            ErrorSeverity::Medium,
        );

        assert!(!error_id.is_empty());
        
        let stats = system.get_statistics();
        assert_eq!(stats.total_errors, 1);
    }

    #[test]
    fn test_recovery_strategy() {
        let system = ErrorRecoverySystem::new();
        
        let strategy = system.get_recovery_strategy(&ErrorType::RateLimitExceeded);
        assert!(strategy.is_some());
    }

    #[test]
    fn test_error_statistics() {
        let system = ErrorRecoverySystem::new();
        
        for _ in 0..3 {
            system.log_error(
                ErrorType::NetworkTimeout,
                "Timeout".to_string(),
                ErrorContext {
                    operation: "test".to_string(),
                    provider: "test".to_string(),
                    model: None,
                    user_id: None,
                    request_id: None,
                },
                ErrorSeverity::Low,
            );
        }

        let stats = system.get_statistics();
        assert_eq!(stats.total_errors, 3);
        assert_eq!(stats.by_severity.low, 3);
    }

    #[test]
    fn test_recovery_tracking() {
        let system = ErrorRecoverySystem::new();
        
        let error_id = system.log_error(
            ErrorType::NetworkTimeout,
            "Test error".to_string(),
            ErrorContext {
                operation: "test".to_string(),
                provider: "test".to_string(),
                model: None,
                user_id: None,
                request_id: None,
            },
            ErrorSeverity::Low,
        );

        system.mark_recovery_attempted(&error_id, true);
        
        let stats = system.get_statistics();
        assert_eq!(stats.recovered, 1);
    }
}
