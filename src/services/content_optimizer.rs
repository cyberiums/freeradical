use serde::{Deserialize, Serialize};
use log::info;

/// Content Optimizer
/// AI-powered content optimization for SEO
pub struct ContentOptimizer;

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_score: f64,
    pub optimized_score: f64,
    pub improvements: Vec<Improvement>,
    pub optimized_suggestions: Vec<String>,
}

/// Individual improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Improvement {
    pub category: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub before: String,
    pub after: String,
}

/// Impact level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

impl ContentOptimizer {
    /// Optimize content for target keyword
    pub async fn optimize_content(
        &self,
        content: &str,
        target_keyword: &str,
        target_audience: &str,
    ) -> Result<OptimizationResult, String> {
        info!("Optimizing content for keyword: {}", target_keyword);

        let original_score = self.score_content(content, target_keyword);
        let improvements = self.generate_improvements(content, target_keyword, target_audience).await?;
        let suggestions = self.generate_suggestions(content, target_keyword);
        
        // Calculate optimized score based on improvements
        let optimized_score = original_score + (improvements.len() as f64 * 5.0).min(40.0);

        Ok(OptimizationResult {
            original_score,
            optimized_score: optimized_score.min(100.0),
            improvements,
            optimized_suggestions: suggestions,
        })
    }

    /// Score existing content
    fn score_content(&self, content: &str, keyword: &str) -> f64 {
        let mut score: f64 = 50.0; // Base score

        // Keyword presence
        let keyword_count = content.to_lowercase().matches(&keyword.to_lowercase()).count();
        if keyword_count > 0 && keyword_count <= 5 {
            score += 15.0;
        } else if keyword_count > 5 {
            score += 5.0; // Over-optimization penalty
        }

        // Content length
        let word_count = content.split_whitespace().count();
        if word_count >= 1000 {
            score += 20.0;
        } else if word_count >= 500 {
            score += 10.0;
        }

        // Structure elements
        if content.contains('#') || content.contains("<h") {
            score += 10.0;
        }
        if content.contains("http") {
            score += 5.0;
        }

        score.min(100.0)
    }

    /// Generate improvements
    async fn generate_improvements(
        &self,
        content: &str,
        keyword: &str,
        _audience: &str,
    ) -> Result<Vec<Improvement>, String> {
        let mut improvements = vec![];

        // Check keyword density
        let keyword_count = content.to_lowercase().matches(&keyword.to_lowercase()).count();
        let word_count = content.split_whitespace().count();
        let density = if word_count > 0 {
            (keyword_count as f64 / word_count as f64) * 100.0
        } else {
            0.0
        };

        if density < 1.0 {
            improvements.push(Improvement {
                category: "Keyword Optimization".to_string(),
                description: format!("Increase keyword density from {:.2}% to ~1.5%", density),
                impact: ImpactLevel::High,
                before: format!("{} occurrences", keyword_count),
                after: format!("{} occurrences recommended", (word_count as f64 * 0.015) as usize),
            });
        }

        // Check title optimization
        if !content.to_lowercase().contains(&keyword.to_lowercase()) {
            improvements.push(Improvement {
                category: "Title".to_string(),
                description: "Include target keyword in title".to_string(),
                impact: ImpactLevel::High,
                before: "No keyword in title".to_string(),
                after: format!("Add '{}' to title", keyword),
            });
        }

        // Check content length
        if word_count < 1000 {
            improvements.push(Improvement {
                category: "Content Length".to_string(),
                description: format!("Expand from {} to 1000+ words", word_count),
                impact: ImpactLevel::Medium,
                before: format!("{} words", word_count),
                after: "1000+ words".to_string(),
            });
        }

        Ok(improvements)
    }

    /// Generate optimization suggestions
    fn generate_suggestions(&self, content: &str, keyword: &str) -> Vec<String> {
        let mut suggestions = vec![];

        // LSI keywords
        suggestions.push(format!(
            "Add related terms: '{}', '{}', '{}'",
            format!("{} guide", keyword),
            format!("{} tips", keyword),
            format!("best {}", keyword)
        ));

        // Internal linking
        if !content.contains("http") {
            suggestions.push("Add 3-5 internal links to related content".to_string());
        }

        // Media
        if !content.contains("<img") && !content.contains("![") {
            suggestions.push("Include 2-3 relevant images with alt text".to_string());
        }

        // Call to action
        suggestions.push("Add clear call-to-action in conclusion".to_string());

        suggestions
    }

    /// Optimize for readability
    pub fn optimize_readability(&self, content: &str) -> ReadabilityOptimization {
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..])
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        let long_sentences: Vec<String> = sentences.iter()
            .filter(|s| s.split_whitespace().count() > 25)
            .map(|s| s.to_string())
            .collect();

        let suggestions = if long_sentences.is_empty() {
            vec!["Readability is good".to_string()]
        } else {
            vec![
                format!("Split {} long sentences (>25 words)", long_sentences.len()),
                "Use transition words for better flow".to_string(),
                "Vary sentence length for engagement".to_string(),
            ]
        };

        ReadabilityOptimization {
            long_sentences,
            readability_score: 75.0, // Simplified
            suggestions,
        }
    }
}

/// Readability optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityOptimization {
    pub long_sentences: Vec<String>,
    pub readability_score: f64,
    pub suggestions: Vec<String>,
}

impl Default for ContentOptimizer {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_content() {
        let optimizer = ContentOptimizer;
        let content = "This is test content about SEO optimization.";
        let result = optimizer.optimize_content(content, "SEO", "marketers").await;

        assert!(result.is_ok());
        let opt = result.unwrap();
        assert!(opt.optimized_score >= opt.original_score);
    }

    #[test]
    fn test_score_content() {
        let optimizer = ContentOptimizer;
        let content = "# SEO Guide\n\nThis comprehensive guide covers SEO best practices.";
        let score = optimizer.score_content(content, "SEO");

        assert!(score > 50.0);
    }

    #[test]
    fn test_optimize_readability() {
        let optimizer = ContentOptimizer;
        let content = "This is a very long sentence that goes on and on and contains way too many words which makes it difficult to read and understand for most people.";
        let result = optimizer.optimize_readability(content);

        assert!(!result.long_sentences.is_empty());
    }
}
