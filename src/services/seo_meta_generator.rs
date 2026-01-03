use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::info;

/// SEO Meta Generator
/// Automatically generates SEO metadata for content
pub struct SEOMetaGenerator;

/// Generated SEO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEOMetadata {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub og_tags: HashMap<String, String>,
    pub twitter_cards: HashMap<String, String>,
    pub canonical_url: String,
}

impl SEOMetaGenerator {
    /// Generate complete SEO metadata
    pub async fn generate_metadata(
        &self,
        content: &str,
        url: &str,
        primary_keyword: &str,
    ) -> Result<SEOMetadata, String> {
        info!("Generating SEO metadata for: {}", url);

        let title = self.generate_title(content, primary_keyword).await?;
        let description = self.generate_description(content, primary_keyword).await?;
        let keywords = self.extract_keywords(content).await?;
        let og_tags = self.generate_og_tags(&title, &description, url).await?;
        let twitter_cards = self.generate_twitter_cards(&title, &description).await?;

        Ok(SEOMetadata {
            title,
            description,
            keywords,
            og_tags,
            twitter_cards,
            canonical_url: url.to_string(),
        })
    }

    /// Generate SEO-optimized title (50-60 chars)
    async fn generate_title(&self, _content: &str, keyword: &str) -> Result<String, String> {
        // TODO: AI-powered title generation
        let title = format!("{} - Complete Guide", keyword);
        Ok(title.chars().take(60).collect())
    }

    /// Generate meta description (150-160 chars)
    async fn generate_description(&self, _content: &str, keyword: &str) -> Result<String, String> {
        // TODO: AI-powered description generation
        let desc = format!("Learn about {} with this comprehensive guide.", keyword);
        Ok(desc.chars().take(160).collect())
    }

    /// Extract relevant keywords
    async fn extract_keywords(&self, _content: &str) -> Result<Vec<String>, String> {
        // TODO: NLP-based keyword extraction
        Ok(vec![
            "primary keyword".to_string(),
            "secondary keyword".to_string(),
            "long-tail keyword".to_string(),
        ])
    }

    /// Generate Open Graph tags
    async fn generate_og_tags(
        &self,
        title: &str,
        description: &str,
        url: &str,
    ) -> Result<HashMap<String, String>, String> {
        let mut tags = HashMap::new();
        tags.insert("og:title".to_string(), title.to_string());
        tags.insert("og:description".to_string(), description.to_string());
        tags.insert("og:url".to_string(), url.to_string());
        tags.insert("og:type".to_string(), "website".to_string());
        Ok(tags)
    }

    /// Generate Twitter Card tags
    async fn generate_twitter_cards(
        &self,
        title: &str,
        description: &str,
    ) -> Result<HashMap<String, String>, String> {
        let mut cards = HashMap::new();
        cards.insert("twitter:card".to_string(), "summary_large_image".to_string());
        cards.insert("twitter:title".to_string(), title.to_string());
        cards.insert("twitter:description".to_string(), description.to_string());
        Ok(cards)
    }

    /// Validate SEO metadata quality
    pub fn validate_metadata(&self, metadata: &SEOMetadata) -> ValidationResult {
        let mut issues = vec![];
        let mut score: f64 = 100.0;

        // Title validation
        if metadata.title.len() < 30 {
            issues.push("Title too short (min 30 chars)".to_string());
            score -= 15.0;
        } else if metadata.title.len() > 60 {
            issues.push("Title too long (max 60 chars)".to_string());
            score -= 10.0;
        }

        // Description validation
        if metadata.description.len() < 120 {
            issues.push("Description too short (min 120 chars)".to_string());
            score -= 15.0;
        } else if metadata.description.len() > 160 {
            issues.push("Description too long (max 160 chars)".to_string());
            score -= 10.0;
        }

        // Keywords validation
        if metadata.keywords.is_empty() {
            issues.push("No keywords provided".to_string());
            score -= 20.0;
        } else if metadata.keywords.len() < 3 {
            issues.push("Add more keywords (min 3)".to_string());
            score -= 10.0;
        }

        ValidationResult {
            is_valid: issues.is_empty(),
            score: score.max(0.0),
            issues,
        }
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub score: f64,
    pub issues: Vec<String>,
}

impl Default for SEOMetaGenerator {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_metadata() {
        let generator = SEOMetaGenerator;
        let content = "This is test content about Rust programming language.";
        let result = generator.generate_metadata(content, "https://example.com", "Rust").await;

        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert!(!metadata.title.is_empty());
        assert!(!metadata.description.is_empty());
        assert!(!metadata.keywords.is_empty());
    }

    #[test]
    fn test_validate_metadata() {
        let generator = SEOMetaGenerator;
        let metadata = SEOMetadata {
            title: "Perfect SEO Title Length Here".to_string(),
            description: "This is a perfect meta description that is long enough to meet SEO requirements but not too long to be cut off in search results.".to_string(),
            keywords: vec!["seo".to_string(), "meta".to_string(), "optimization".to_string()],
            og_tags: HashMap::new(),
            twitter_cards: HashMap::new(),
            canonical_url: "https://example.com".to_string(),
        };

        let result = generator.validate_metadata(&metadata);
        assert!(result.score > 80.0);
    }

    #[test]
    fn test_validate_short_title() {
        let generator = SEOMetaGenerator;
        let metadata = SEOMetadata {
            title: "Short".to_string(),
            description: "Valid description that meets the minimum character requirements for SEO best practices and search engine guidelines.".to_string(),
            keywords: vec!["test".to_string()],
            og_tags: HashMap::new(),
            twitter_cards: HashMap::new(),
            canonical_url: "https://example.com".to_string(),
        };

        let result = generator.validate_metadata(&metadata);
        assert!(!result.issues.is_empty());
        assert!(result.score < 100.0);
    }
}
