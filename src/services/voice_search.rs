use serde::{Deserialize, Serialize};
use log::info;

/// Voice Search Optimizer
/// Optimize content for voice search and virtual assistants
pub struct VoiceSearchOptimizer;

/// Voice search optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceOptimization {
    pub original_content: String,
    pub voice_friendly_version: String,
    pub featured_snippet_candidates: Vec<String>,
    pub qa_pairs: Vec<QAPair>,
    pub conversational_score: f64,
    pub recommendations: Vec<String>,
}

/// Question-Answer pair for voice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAPair {
    pub question: String,
    pub answer: String,
    pub word_count: usize,
}

impl VoiceSearchOptimizer {
    /// Optimize content for voice search
    pub async fn optimize_for_voice(
        &self,
        content: &str,
    ) -> Result<VoiceOptimization, String> {
        info!("Optimizing content for voice search");

        let qa_pairs = self.extract_voice_qa_pairs(content).await?;
        let snippets = self.generate_featured_snippets(&qa_pairs);
        let voice_version = self.create_conversational_version(content, &qa_pairs).await?;
        let conversational_score = self.score_conversational_quality(content);
        let recommendations = self.generate_voice_recommendations(conversational_score, &qa_pairs);

        Ok(VoiceOptimization {
            original_content: content.to_string(),
            voice_friendly_version: voice_version,
            featured_snippet_candidates: snippets,
            qa_pairs,
            conversational_score,
            recommendations,
        })
    }

    /// Extract Q&A pairs optimized for voice
    async fn extract_voice_qa_pairs(&self, content: &str) -> Result<Vec<QAPair>, String> {
        // TODO: Use NLP to extract natural questions
        let mut pairs = vec![];

        // Look for question sentences
        for sentence in content.split('.') {
            let trimmed = sentence.trim();
            if trimmed.contains('?') {
                pairs.push(QAPair {
                    question: trimmed.to_string(),
                    answer: "See detailed explanation in content".to_string(),
                    word_count: trimmed.split_whitespace().count(),
                });
            }
        }

        // Generate common voice queries
        pairs.push(QAPair {
            question: "What is this about?".to_string(),
            answer: content.split_whitespace().take(30).collect::<Vec<&str>>().join(" "),
            word_count: 30,
        });

        Ok(pairs)
    }

    /// Generate featured snippet candidates
    fn generate_featured_snippets(&self, qa_pairs: &[QAPair]) -> Vec<String> {
        qa_pairs.iter()
            .filter(|qa| qa.word_count <= 50) // Optimal snippet length
            .map(|qa| format!("Q: {}\nA: {}", qa.question, qa.answer))
            .collect()
    }

    /// Create conversational version
    async fn create_conversational_version(
        &self,
        content: &str,
        qa_pairs: &[QAPair],
    ) -> Result<String, String> {
        let mut conversational = String::new();

        // Add natural language intro
        conversational.push_str("Here's what you need to know:\n\n");

        // Add Q&A section
        for qa in qa_pairs {
            conversational.push_str(&format!("**{}**\n{}\n\n", qa.question, qa.answer));
        }

        // Add original content with conversational markers
        conversational.push_str("For more details:\n");
        conversational.push_str(content);

        Ok(conversational)
    }

    /// Score conversational quality
    fn score_conversational_quality(&self, content: &str) -> f64 {
        let mut score: f64 = 50.0; // Base score

        // Check for conversational elements
        let lower = content.to_lowercase();
        
        // Questions (good for voice)
        let question_count = content.matches('?').count();
        score += (question_count as f64 * 5.0).min(20.0);

        // Conversational words
        let conversational_words = vec!["you", "your", "how", "why", "what", "when", "where"];
        let mut found_conversational = 0;
        for word in conversational_words {
            if lower.contains(word) {
                found_conversational += 1;
            }
        }
        score += (found_conversational as f64 * 3.0).min(15.0);

        // Short sentences (easier for voice)
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        let short_sentences = sentences.iter()
            .filter(|s| s.split_whitespace().count() <= 20)
            .count();
        let short_ratio = short_sentences as f64 / sentences.len().max(1) as f64;
        score += short_ratio * 15.0;

        score.min(100.0)
    }

    /// Generate voice optimization recommendations
    fn generate_voice_recommendations(&self, score: f64, qa_pairs: &[QAPair]) -> Vec<String> {
        let mut recommendations = vec![];

        if score < 60.0 {
            recommendations.push("Use more conversational language (you, your, etc.)".to_string());
            recommendations.push("Add more question-answer pairs".to_string());
        }

        if qa_pairs.len() < 3 {
            recommendations.push("Create at least 5 natural Q&A pairs for voice queries".to_string());
        }

        let long_answers = qa_pairs.iter().filter(|qa| qa.word_count > 50).count();
        if long_answers > qa_pairs.len() / 2 {
            recommendations.push("Keep answers concise (under 50 words) for voice assistants".to_string());
        }

        recommendations.push("Add FAQ schema markup for voice search visibility".to_string());
        recommendations.push("Optimize for local voice queries (near me, open now)".to_string());

        if score >= 80.0 {
            recommendations.push("Excellent voice optimization! Monitor voice search traffic.".to_string());
        }

        recommendations
    }

    /// Analyze voice search potential
    pub fn analyze_voice_potential(&self, content: &str, keywords: Vec<String>) -> VoiceSearchPotential {
        let question_keywords: Vec<String> = keywords.iter()
            .filter(|k| {
                let lower = k.to_lowercase();
                lower.starts_with("how") || 
                lower.starts_with("what") || 
                lower.starts_with("why") ||
                lower.starts_with("when") ||
                lower.starts_with("where")
            })
            .cloned()
            .collect();

        let local_intent = keywords.iter()
            .any(|k| {
                let lower = k.to_lowercase();
                lower.contains("near me") || 
                lower.contains("nearby") ||
                lower.contains("open now")
            });

        VoiceSearchPotential {
            question_based_keywords: question_keywords.len(),
            has_local_intent: local_intent,
            voice_readiness_score: self.score_conversational_quality(content),
            recommended_features: vec![
                "FAQ schema".to_string(),
                "Speakable schema".to_string(),
                "Local business schema".to_string(),
            ],
        }
    }
}

/// Voice search potential analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSearchPotential {
    pub question_based_keywords: usize,
    pub has_local_intent: bool,
    pub voice_readiness_score: f64,
    pub recommended_features: Vec<String>,
}

impl Default for VoiceSearchOptimizer {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_for_voice() {
        let optimizer = VoiceSearchOptimizer;
        let content = "What is SEO? SEO stands for Search Engine Optimization. How does it work? It helps websites rank higher.";
        
        let result = optimizer.optimize_for_voice(content).await;
        assert!(result.is_ok());
        
        let opt = result.unwrap();
        assert!(!opt.qa_pairs.is_empty());
        assert!(opt.conversational_score > 0.0);
    }

    #[test]
    fn test_score_conversational_quality() {
        let optimizer = VoiceSearchOptimizer;
        let content = "What is your question? How can I help you today?";
        
        let score = optimizer.score_conversational_quality(content);
        assert!(score > 60.0); // Should score well with questions and "you"
    }

    #[test]
    fn test_analyze_voice_potential() {
        let optimizer = VoiceSearchOptimizer;
        let keywords = vec![
            "how to optimize seo".to_string(),
            "what is seo".to_string(),
            "seo near me".to_string(),
        ];
        
        let potential = optimizer.analyze_voice_potential("test content", keywords);
        assert_eq!(potential.question_based_keywords, 2);
        assert!(potential.has_local_intent);
    }
}
