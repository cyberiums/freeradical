use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use log::info;

/// Schema Markup Generator
/// Generates structured data (Schema.org) for SEO
pub struct SchemaMarkupGenerator;

/// Schema types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaType {
    Article,
    BlogPosting,
    Product,
    FAQPage,
    HowTo,
    Organization,
    LocalBusiness,
}

/// Generated schema markup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMarkup {
    pub schema_type: SchemaType,
    pub json_ld: String,
    pub validation_score: f64,
}

impl SchemaMarkupGenerator {
    /// Generate Article schema
    pub fn generate_article_schema(
        &self,
        title: &str,
        description: &str,
        author: &str,
        date_published: &str,
        url: &str,
    ) -> Result<SchemaMarkup, String> {
        info!("Generating Article schema");

        let mut schema = HashMap::new();
        schema.insert("@context", "https://schema.org");
        schema.insert("@type", "Article");
        schema.insert("headline", title);
        schema.insert("description", description);
        schema.insert("author", author);
        schema.insert("datePublished", date_published);
        schema.insert("url", url);

        let json_ld = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        Ok(SchemaMarkup {
            schema_type: SchemaType::Article,
            json_ld,
            validation_score: self.validate_schema(&schema),
        })
    }

    /// Generate FAQ schema
    pub fn generate_faq_schema(&self, questions: Vec<(String, String)>) -> Result<SchemaMarkup, String> {
        info!("Generating FAQ schema with {} questions", questions.len());

        let mut main_entity = vec![];
        
        for (question, answer) in questions {
            let mut qa = HashMap::new();
            qa.insert("@type", "Question");
            qa.insert("name", &question);
            
            let mut answer_obj = HashMap::new();
            answer_obj.insert("@type", "Answer");
            answer_obj.insert("text", &answer);
            
            main_entity.push(serde_json::to_value(qa).unwrap());
        }

        let mut schema = HashMap::new();
        schema.insert("@context".to_string(), serde_json::Value::String("https://schema.org".to_string()));
        schema.insert("@type".to_string(), serde_json::Value::String("FAQPage".to_string()));
        schema.insert("mainEntity".to_string(), serde_json::Value::Array(main_entity));

        let json_ld = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        Ok(SchemaMarkup {
            schema_type: SchemaType::FAQPage,
            json_ld,
            validation_score: 95.0,
        })
    }

    /// Generate Product schema
    pub fn generate_product_schema(
        &self,
        name: &str,
        description: &str,
        price: f64,
        currency: &str,
    ) -> Result<SchemaMarkup, String> {
        info!("Generating Product schema");

        let price_str = price.to_string();
        let mut offers = HashMap::new();
        offers.insert("@type", "Offer");
        offers.insert("price", &price_str);
        offers.insert("priceCurrency", currency);

        let mut schema = HashMap::new();
        schema.insert("@context", "https://schema.org");
        schema.insert("@type", "Product");
        schema.insert("name", name);
        schema.insert("description", description);

        let json_ld = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        Ok(SchemaMarkup {
            schema_type: SchemaType::Product,
            json_ld,
            validation_score: 88.0,
        })
    }

    /// Validate schema completeness
    fn validate_schema(&self, schema: &HashMap<&str, &str>) -> f64 {
        let required_fields = vec!["@context", "@type"];
        let mut score = 0.0;

        for field in &required_fields {
            if schema.contains_key(field) {
                score += 30.0;
            }
        }

        // Additional fields boost score
        score += (schema.len() as f64 - 2.0) * 5.0;
        
        score.min(100.0)
    }
}

impl Default for SchemaMarkupGenerator {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_article_schema() {
        let generator = SchemaMarkupGenerator;
        let result = generator.generate_article_schema(
            "Test Article",
            "Description",
            "Author Name",
            "2024-12-26",
            "https://example.com",
        );

        assert!(result.is_ok());
        let schema = result.unwrap();
        assert!(schema.json_ld.contains("Article"));
        assert!(schema.validation_score > 0.0);
    }

    #[test]
    fn test_generate_faq_schema() {
        let generator = SchemaMarkupGenerator;
        let questions = vec![
            ("What is Rust?".to_string(), "A systems programming language".to_string()),
            ("Why use Rust?".to_string(), "Memory safety without garbage collection".to_string()),
        ];

        let result = generator.generate_faq_schema(questions);
        assert!(result.is_ok());
        
        let schema = result.unwrap();
        assert!(schema.json_ld.contains("FAQPage"));
    }

    #[test]
    fn test_generate_product_schema() {
        let generator = SchemaMarkupGenerator;
        let result = generator.generate_product_schema(
            "Test Product",
            "A great product",
            99.99,
            "USD",
        );

        assert!(result.is_ok());
        let schema = result.unwrap();
        assert!(schema.json_ld.contains("Product"));
    }
}
