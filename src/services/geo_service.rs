use serde::{Deserialize, Serialize};
use log::info;

/// Generative Engine Optimization (GEO) Service
/// Optimizes content for generative AI engines (GPT, Claude, Gemini, etc.)
pub struct GEOService;

/// GEO optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEOOptimization {
    pub original_content: String,
    pub optimized_content: String,
    pub citation_score: f64,
    pub context_quality: f64,
    pub source_attribution: Vec<SourceAttribution>,
    pub recommendations: Vec<String>,
}

/// Source attribution for citations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub claim: String,
    pub source_url: Option<String>,
    pub credibility_score: f64,
}

impl GEOService {
    /// Optimize content for generative engines
    pub async fn optimize_for_generative_ai(
        &self,
        content: &str,
        _domain: &str,
    ) -> Result<GEOOptimization, String> {
        info!("Optimizing content for generative AI engines");

        // Add source attribution
        let attributions = self.extract_attributions(content).await?;

        // Optimize context structure
        let optimized = self.add_context_markers(content).await?;

        // Calculate scores
        let citation_score = self.calculate_citation_score(&attributions);
        let context_quality = self.calculate_context_quality(content);

        // Generate recommendations
        let recommendations = self.generate_recommendations(citation_score, context_quality);

        Ok(GEOOptimization {
            original_content: content.to_string(),
            optimized_content: optimized,
            citation_score,
            context_quality,
            source_attribution: attributions,
            recommendations,
        })
    }

    /// Extract source attributions
    async fn extract_attributions(&self, _content: &str) -> Result<Vec<SourceAttribution>, String> {
        // TODO: AI-powered claim extraction and source identification

        Ok(vec![
            SourceAttribution {
                claim: "Key statement from content".to_string(),
                source_url: Some("https://example.com/source".to_string()),
                credibility_score: 0.85,
            },
        ])
    }

    /// Add context markers for AI understanding
    async fn add_context_markers(&self, content: &str) -> Result<String, String> {
        let mut optimized = String::new();

        // Add topic context
        optimized.push_str("<!-- CONTEXT: Primary Topic -->\n");
        optimized.push_str(content);
        optimized.push_str("\n<!-- END CONTEXT -->\n");

        Ok(optimized)
    }

    /// Calculate citation worthiness score
    fn calculate_citation_score(&self, attributions: &[SourceAttribution]) -> f64 {
        if attributions.is_empty() {
            return 0.0;
        }

        let avg_credibility: f64 = attributions.iter()
            .map(|a| a.credibility_score)
            .sum::<f64>() / attributions.len() as f64;

        (attributions.len() as f64 * 10.0 * avg_credibility).min(100.0)
    }

    /// Calculate context quality
    fn calculate_context_quality(&self, content: &str) -> f64 {
        let word_count = content.split_whitespace().count();
        let has_headers = content.contains('#');
        let has_lists = content.contains('-') || content.contains('*');

        let mut score = 0.0;

        // Length score (40%)
        score += if word_count > 500 { 40.0 } else { word_count as f64 / 500.0 * 40.0 };

        // Structure score (30%)
        if has_headers { score += 15.0; }
        if has_lists { score += 15.0; }

        // Completeness (30%)
        score += 30.0; // Assume complete for now

        score.min(100.0)
    }

    /// Generate optimization recommendations
    fn generate_recommendations(&self, citation_score: f64, context_quality: f64) -> Vec<String> {
        let mut recommendations = vec![];

        if citation_score < 50.0 {
            recommendations.push("Add more source citations to increase credibility".to_string());
        }

        if context_quality < 70.0 {
            recommendations.push("Improve content structure with headers and lists".to_string());
        }

        if context_quality < 50.0 {
            recommendations.push("Expand content to provide more comprehensive information".to_string());
        }

        recommendations
    }

    /// Analyze generative engine visibility
    pub async fn analyze_geo_performance(
        &self,
        url: &str,
    ) -> Result<GEOPerformance, String> {
        info!("Analyzing GEO performance for: {}", url);

        // TODO: Check actual citations in GPT, Claude, Gemini responses

        Ok(GEOPerformance {
            url: url.to_string(),
            citation_frequency: 0.42,
            engines: vec![
                EnginePerformance {
                    engine: "GPT-4".to_string(),
                    citation_rate: 0.45,
                    average_position: 3,
                },
                EnginePerformance {
                    engine: "Claude".to_string(),
                    citation_rate: 0.52,
                    average_position: 2,
                },
                EnginePerformance {
                    engine: "Gemini".to_string(),
                    citation_rate: 0.28,
                    average_position: 5,
                },
            ],
        })
    }
}

/// GEO performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEOPerformance {
    pub url: String,
    pub citation_frequency: f64,
    pub engines: Vec<EnginePerformance>,
}

/// Engine-specific performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnginePerformance {
    pub engine: String,
    pub citation_rate: f64,
    pub average_position: u32,
}

impl Default for GEOService {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_for_generative_ai() {
        let service = GEOService;
        let content = "# Test Content\n\nThis is a test article about Rust programming.";
        let result = service.optimize_for_generative_ai(content, "example.com").await;

        assert!(result.is_ok());
        let optimization = result.unwrap();
        assert!(optimization.citation_score >= 0.0);
        assert!(optimization.context_quality > 0.0);
    }

    #[tokio::test]
    async fn test_analyze_geo_performance() {
        let service = GEOService;
        let result = service.analyze_geo_performance("https://example.com").await;

        assert!(result.is_ok());
        let performance = result.unwrap();
        assert!(!performance.engines.is_empty());
    }

    #[test]
    fn test_calculate_scores() {
        let service = GEOService;
        
        let attributions = vec![
            SourceAttribution {
                claim: "Test claim".to_string(),
                source_url: Some("url".to_string()),
                credibility_score: 0.8,
            },
        ];

        let citation_score = service.calculate_citation_score(&attributions);
        assert!(citation_score > 0.0);

        let context_quality = service.calculate_context_quality("# Header\n\nContent here");
        assert!(context_quality > 0.0);
    }
}
