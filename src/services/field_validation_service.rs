// Field Validation Service
// Validates field content based on field type and validation rules

use serde::{Deserialize, Serialize};
use crate::models::field_type_enum::FieldType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRule {
    pub required: Option<bool>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,  // regex pattern
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<String>>,  // For select/multi_select
}

#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Validation error in {}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Validate field content based on type and rules
pub fn validate_field(
    field_name: &str,
    field_type: &FieldType,
    content: &str,
    rules_json: Option<&str>,
) -> Result<(), ValidationError> {
    // Parse validation rules if provided
    let rules: Option<ValidationRule> = rules_json
        .and_then(|json| serde_json::from_str(json).ok());
    
    // Required check
    if let Some(ref r) = rules {
        if r.required == Some(true) && content.trim().is_empty() {
            return Err(ValidationError {
                field: field_name.to_string(),
                message: "Field is required".to_string(),
            });
        }
    }
    
    // Type-specific validation
    match field_type {
        FieldType::Text | FieldType::Textarea => {
            validate_text(field_name, content, rules.as_ref())
        }
        FieldType::Wysiwyg => {
            validate_html(field_name, content, rules.as_ref())
        }
        FieldType::Json => {
            validate_json(field_name, content, rules.as_ref())
        }
        FieldType::Number => {
            validate_number(field_name, content, rules.as_ref())
        }
        FieldType::Boolean => {
            validate_boolean(field_name, content)
        }
        FieldType::Date => {
            validate_date(field_name, content)
        }
        FieldType::Datetime => {
            validate_datetime(field_name, content)
        }
        FieldType::FileReference | FieldType::PageReference => {
            validate_uuid(field_name, content)
        }
        FieldType::Select => {
            validate_select(field_name, content, rules.as_ref())
        }
        FieldType::MultiSelect => {
            validate_multi_select(field_name, content, rules.as_ref())
        }
    }
}

fn validate_text(field_name: &str, content: &str, rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    if let Some(rules) = rules {
        if let Some(min) = rules.min_length {
            if content.len() < min {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: format!("Minimum length is {} characters", min),
                });
            }
        }
        
        if let Some(max) = rules.max_length {
            if content.len() > max {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: format!("Maximum length is {} characters", max),
                });
            }
        }
        
        if let Some(ref pattern) = rules.pattern {
            let regex = regex::Regex::new(pattern).map_err(|_| ValidationError {
                field: field_name.to_string(),
                message: "Invalid regex pattern in validation rules".to_string(),
            })?;
            
            if !regex.is_match(content) {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: "Content does not match required pattern".to_string(),
                });
            }
        }
    }
    
    Ok(())
}

fn validate_html(field_name: &str, content: &str, rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    // Basic HTML validation - check for balanced tags
    // For production, consider using ammonia or bleach for sanitization
    
    validate_text(field_name, content, rules)?;
    
    // Simple check for obviously malformed HTML
    if content.contains("<script>") || content.contains("javascript:") {
        return Err(ValidationError {
            field: field_name.to_string(),
            message: "Potentially unsafe HTML content detected".to_string(),
        });
    }
    
    Ok(())
}

fn validate_json(field_name: &str, content: &str, _rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    serde_json::from_str::<serde_json::Value>(content).map_err(|e| ValidationError {
        field: field_name.to_string(),
        message: format!("Invalid JSON: {}", e),
    })?;
    
    Ok(())
}

fn validate_number(field_name: &str, content: &str, rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    let num: f64 = content.parse().map_err(|_| ValidationError {
        field: field_name.to_string(),
        message: "Must be a valid number".to_string(),
    })?;
    
    if let Some(rules) = rules {
        if let Some(min) = rules.min_value {
            if num < min {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: format!("Minimum value is {}", min),
                });
            }
        }
        
        if let Some(max) = rules.max_value {
            if num > max {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: format!("Maximum value is {}", max),
                });
            }
        }
    }
    
    Ok(())
}

fn validate_boolean(field_name: &str, content: &str) -> Result<(), ValidationError> {
    if content != "true" && content != "false" && content != "0" && content != "1" {
        return Err(ValidationError {
            field: field_name.to_string(),
            message: "Must be 'true', 'false', '0', or '1'".to_string(),
        });
    }
    Ok(())
}

fn validate_date(field_name: &str, content: &str) -> Result<(), ValidationError> {
    chrono::NaiveDate::parse_from_str(content, "%Y-%m-%d").map_err(|_| ValidationError {
        field: field_name.to_string(),
        message: "Invalid date format. Use YYYY-MM-DD".to_string(),
    })?;
    Ok(())
}

fn validate_datetime(field_name: &str, content: &str) -> Result<(), ValidationError> {
    chrono::NaiveDateTime::parse_from_str(content, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| chrono::DateTime::parse_from_rfc3339(content).map(|_| chrono::NaiveDateTime::default()))
        .map_err(|_| ValidationError {
            field: field_name.to_string(),
            message: "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS or RFC3339".to_string(),
        })?;
    Ok(())
}

fn validate_uuid(field_name: &str, content: &str) -> Result<(), ValidationError> {
    uuid::Uuid::parse_str(content).map_err(|_| ValidationError {
        field: field_name.to_string(),
        message: "Must be a valid UUID".to_string(),
    })?;
    Ok(())
}

fn validate_select(field_name: &str, content: &str, rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    if let Some(rules) = rules {
        if let Some(ref allowed) = rules.allowed_values {
            if !allowed.contains(&content.to_string()) {
                return Err(ValidationError {
                    field: field_name.to_string(),
                    message: format!("Value must be one of: {}", allowed.join(", ")),
                });
            }
        }
    }
    Ok(())
}

fn validate_multi_select(field_name: &str, content: &str, rules: Option<&ValidationRule>) -> Result<(), ValidationError> {
    // Expect JSON array
    let values: Vec<String> = serde_json::from_str(content).map_err(|_| ValidationError {
        field: field_name.to_string(),
        message: "Multi-select must be a JSON array of strings".to_string(),
    })?;
    
    if let Some(rules) = rules {
        if let Some(ref allowed) = rules.allowed_values {
            for value in &values {
                if !allowed.contains(value) {
                    return Err(ValidationError {
                        field: field_name.to_string(),
                        message: format!("All values must be from: {}", allowed.join(", ")),
                    });
                }
            }
        }
    }
    
    Ok(())
}
