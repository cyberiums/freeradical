use serde::{Deserialize, Serialize};
use log::info;

/// Content Quality Analyzer
/// Analyzes content quality for SEO and readability
pub struct ContentQualityAnalyzer;

/// Quality analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAnalysis {
    pub overall_score: f64,
    pub readability_score: f64,
    pub seo_score: f64,
    pub engagement_score: f64,
    pub recommendations: Vec<String>,
}

/// Readability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityMetrics {
    pub flesch_reading_ease: f64,
    pub flesch_kincaid_grade: f64,
    pub word_count: usize,
    pub sentence_count: usize,
    pub avg_sentence_length: f64,
}

impl ContentQualityAnalyzer {
    /// Analyze content quality
    pub fn analyze(&self, content: &str) -> QualityAnalysis {
        info!("Analyzing content quality");

        let readability = self.calculate_readability(content);
        let seo = self.calculate_seo_score(content);
        let engagement = self.calculate_engagement_score(content);
        
        let overall = (readability + seo + engagement) / 3.0;
        let recommendations = self.generate_recommendations(readability, seo, engagement);

        QualityAnalysis {
            overall_score: overall,
            readability_score: readability,
            seo_score: seo,
            engagement_score: engagement,
            recommendations,
        }
    }

    /// Calculate readability score
    fn calculate_readability(&self, content: &str) -> f64 {
        let metrics = self.get_readability_metrics(content);
        
        // Simplified Flesch Reading Ease calculation
        // Score: 0-100 (higher is easier to read)
        let score = 206.835 - 1.015 * metrics.avg_sentence_length - 84.6 * self.syllables_per_word(content);
        
        // Normalize to 0-100
        score.max(0.0).min(100.0)
    }

    /// Get detailed readability metrics
    fn get_readability_metrics(&self, content: &str) -> ReadabilityMetrics {
        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count = words.len();
        
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        let sentence_count = sentences.iter().filter(|s| !s.trim().is_empty()).count().max(1);
        
        let avg_sentence_length = word_count as f64 / sentence_count as f64;

        ReadabilityMetrics {
            flesch_reading_ease: 0.0, // Calculated separately
            flesch_kincaid_grade: 0.0,
            word_count,
            sentence_count,
            avg_sentence_length,
        }
    }

    /// Estimate syllables per word (simplified)
    fn syllables_per_word(&self, _content: &str) -> f64 {
        // Simplified: estimate ~1.5 syllables per word for English
        1.5
    }

    /// Calculate SEO score
    fn calculate_seo_score(&self, content: &str) -> f64 {
        let word_count = content.split_whitespace().count();
        let has_headers = content.contains('#') || content.contains("<h");
        let has_links = content.contains("http");
        let has_lists = content.contains('-') || content.contains('*') || content.contains("<li");

        let mut score = 0.0;

        // Word count (ideal 1000-2000 words)
        if word_count >= 1000 && word_count <= 2000 {
            score += 30.0;
        } else if word_count >= 500 {
            score += 15.0;
        }

        // Structure elements
        if has_headers { score += 25.0; }
        if has_links { score += 20.0; }
        if has_lists { score += 25.0; }

        score
    }

    /// Calculate engagement score
    fn calculate_engagement_score(&self, content: &str) -> f64 {
        let word_count = content.split_whitespace().count();
        let has_questions = content.contains('?');
        let has_numbers = content.chars().any(|c| c.is_numeric());
        let has_quotes = content.contains('"') || content.contains('\'');

        let mut score: f64 = 50.0; // Base score

        // Engagement elements
        if has_questions { score += 15.0; }
        if has_numbers { score += 15.0; }
        if has_quotes { score += 10.0; }
        
        // Length engagement (too short or too long reduces engagement)
        if word_count >= 300 && word_count <= 1500 {
            score += 10.0;
        }

        score.min(100.0)
    }

    /// Generate actionable recommendations
    fn generate_recommendations(&self, readability: f64, seo: f64, engagement: f64) -> Vec<String> {
        let mut recommendations = vec![];

        if readability < 50.0 {
            recommendations.push("Simplify sentence structure for better readability".to_string());
            recommendations.push("Use shorter sentences and common words".to_string());
        }

        if seo < 60.0 {
            recommendations.push("Add more headers (H2, H3) to structure content".to_string());
            recommendations.push("Include internal and external links".to_string());
            recommendations.push("Expand content to 1000+ words for better SEO".to_string());
        }

        if engagement < 60.0 {
            recommendations.push("Add questions to engage readers".to_string());
            recommendations.push("Include specific numbers and statistics".to_string());
            recommendations.push("Use quotes or examples to illustrate points".to_string());
        }

        if readability > 80.0 && seo > 80.0 && engagement > 80.0 {
            recommendations.push("Excellent! Content quality is high across all metrics".to_string());
        }

        recommendations
    }
}

impl Default for ContentQualityAnalyzer {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_quality() {
        let analyzer = ContentQualityAnalyzer;
        let content = "This is a test article. It has multiple sentences. Some questions? And numbers like 42.";
        
        let result = analyzer.analyze(content);
        assert!(result.overall_score > 0.0);
        assert!(result.overall_score <= 100.0);
    }

    #[test]
    fn test_readability_metrics() {
        let analyzer = ContentQualityAnalyzer;
        let content = "This is a sentence. This is another sentence.";
        
        let metrics = analyzer.get_readability_metrics(content);
        assert_eq!(metrics.word_count, 8);
        assert_eq!(metrics.sentence_count, 2);
    }

    #[test]
    fn test_recommendations() {
        let analyzer = ContentQualityAnalyzer;
        let content = "Short.";
        
        let result = analyzer.analyze(content);
        assert!(!result.recommendations.is_empty());
    }
}
