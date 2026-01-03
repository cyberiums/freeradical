use serde::{Deserialize, Serialize};
use log::info;

/// Link Builder & Analyzer
/// Internal/external link optimization for SEO
pub struct LinkBuilder;

/// Link analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAnalysis {
    pub internal_links: Vec<LinkInfo>,
    pub external_links: Vec<LinkInfo>,
    pub broken_links: Vec<String>,
    pub link_score: f64,
    pub recommendations: Vec<String>,
}

/// Individual link information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub url: String,
    pub anchor_text: String,
    pub is_dofollow: bool,
    pub context: String,
}

impl LinkBuilder {
    /// Analyze links in content
    pub fn analyze_links(&self, content: &str, base_url: &str) -> LinkAnalysis {
        info!("Analyzing links in content");

        let internal_links = self.extract_internal_links(content, base_url);
        let external_links = self.extract_external_links(content, base_url);
        let broken_links = self.find_broken_links(content);
        let link_score = self.calculate_link_score(&internal_links, &external_links);
        let recommendations = self.generate_link_recommendations(&internal_links, &external_links);

        LinkAnalysis {
            internal_links,
            external_links,
            broken_links,
            link_score,
            recommendations,
        }
    }

    /// Extract internal links
    fn extract_internal_links(&self, content: &str, base_url: &str) -> Vec<LinkInfo> {
        let mut links = vec![];

        // Simple regex-like extraction (simplified for demo)
        if content.contains(base_url) {
            links.push(LinkInfo {
                url: format!("{}/page1", base_url),
                anchor_text: "Related Article".to_string(),
                is_dofollow: true,
                context: "See this related article for more information".to_string(),
            });
        }

        links
    }

    /// Extract external links
    fn extract_external_links(&self, content: &str, base_url: &str) -> Vec<LinkInfo> {
        let mut links = vec![];

        // Check for common external domains
        if content.contains("http") && !content.contains(base_url) {
            links.push(LinkInfo {
                url: "https://external-source.com".to_string(),
                anchor_text: "External Reference".to_string(),
                is_dofollow: true,
                context: "According to external research".to_string(),
            });
        }

        links
    }

    /// Find broken links (placeholder)
    fn find_broken_links(&self, _content: &str) -> Vec<String> {
        // TODO: Actual HTTP checks for broken links
        vec![]
    }

    /// Calculate link quality score
    fn calculate_link_score(&self, internal: &[LinkInfo], external: &[LinkInfo]) -> f64 {
        let mut score = 0.0;

        // Internal links (40 points)
        let internal_count = internal.len();
        score += if internal_count >= 3 && internal_count <= 10 {
            40.0
        } else if internal_count > 0 {
            20.0
        } else {
            0.0
        };

        // External links (30 points)
        let external_count = external.len();
        let authoritative_external = external.iter().filter(|l| l.is_dofollow).count();
        score += if authoritative_external >= 2 && authoritative_external <= 5 {
            30.0
        } else if authoritative_external > 0 {
            15.0
        } else {
            0.0
        };

        // Dofollow ratio (30 points)
        let total_links = internal_count + external_count;
        if total_links > 0 {
            let dofollow_count = internal.iter().chain(external.iter())
                .filter(|l| l.is_dofollow)
                .count();
            let dofollow_ratio = dofollow_count as f64 / total_links as f64;
            score += dofollow_ratio * 30.0;
        }

        score.min(100.0)
    }

    /// Generate link building recommendations
    fn generate_link_recommendations(&self, internal: &[LinkInfo], external: &[LinkInfo]) -> Vec<String> {
        let mut recommendations = vec![];

        if internal.len() < 3 {
            recommendations.push("Add more internal links (aim for 3-10 relevant links)".to_string());
        }

        if internal.len() > 10 {
            recommendations.push("Reduce internal links to avoid over-optimization".to_string());
        }

        if external.is_empty() {
            recommendations.push("Add 2-3 authoritative external references to boost credibility".to_string());
        }

        if external.len() > 5 {
            recommendations.push("Too many external links may dilute page authority".to_string());
        }

        let dofollow_count = internal.iter().chain(external.iter())
            .filter(|l| l.is_dofollow)
            .count();
        if dofollow_count == 0 && !(internal.is_empty() && external.is_empty()) {
            recommendations.push("Consider making some links dofollow for SEO value".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Link profile looks good!".to_string());
        }

        recommendations
    }

    /// Suggest internal link opportunities
    pub fn suggest_internal_links(&self, content: &str, site_pages: Vec<String>) -> Vec<LinkSuggestion> {
        info!("Analyzing content for internal link opportunities");
        
        let mut suggestions = vec![];
        let content_lower = content.to_lowercase();

        for page in site_pages {
            // Extract keywords from page path (remove slashes, split on hyphens)
            let page_topic = page.split('/').last().unwrap_or("");
            
            // Split on hyphens to get individual keywords
            let keywords: Vec<&str> = page_topic.split('-').collect();
            
            // Check if any keyword appears in content
            let mut has_match = false;
            for keyword in &keywords {
                if !keyword.is_empty() && content_lower.contains(&keyword.to_lowercase()) {
                    has_match = true;
                    break;
                }
            }
            
            if has_match {
                suggestions.push(LinkSuggestion {
                    target_url: page.clone(),
                    suggested_anchor: page_topic.replace('-', " "),
                    relevance_score: 0.85,
                    placement_context: "Related topic mentioned in content".to_string(),
                });
            }
        }

        suggestions
    }
}

/// Link suggestion for internal linking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSuggestion {
    pub target_url: String,
    pub suggested_anchor: String,
    pub relevance_score: f64,
    pub placement_context: String,
}

impl Default for LinkBuilder {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_links() {
        let builder = LinkBuilder;
        let content = "Check out https://example.com for more info. Also see https://example.com/page1";
        let result = builder.analyze_links(content, "https://example.com");

        assert!(result.link_score >= 0.0);
        assert!(result.link_score <= 100.0);
    }

    #[test]
    fn test_suggest_internal_links() {
        let builder = LinkBuilder;
        let content = "Learn about rust programming and web development";
        let pages = vec![
            "/blog/rust-tutorial".to_string(),
            "/guide/web-development".to_string(),
        ];

        let suggestions = builder.suggest_internal_links(content, pages);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_link_recommendations() {
        let builder = LinkBuilder;
        let analysis = builder.analyze_links("Simple content", "https://example.com");
        
        assert!(!analysis.recommendations.is_empty());
    }
}
