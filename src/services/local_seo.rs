use serde::{Deserialize, Serialize};
use log::info;

/// Local SEO Optimizer
/// Optimize for local search and Google Business Profile
pub struct LocalSEOOptimizer;

/// Local SEO optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalSEOOptimization {
    pub business_name: String,
    pub location: Location,
    pub local_score: f64,
    pub gmb_optimization: GMBOptimization,
    pub local_citations: Vec<Citation>,
    pub recommendations: Vec<String>,
}

/// Business location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Google My Business optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GMBOptimization {
    pub profile_completeness: f64,
    pub review_count: usize,
    pub average_rating: f64,
    pub response_rate: f64,
    pub post_frequency: f64,
}

/// Local citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub platform: String,
    pub url: String,
    pub nap_consistency: bool, // Name, Address, Phone consistency
}

impl LocalSEOOptimizer {
    /// Optimize for local search
    pub async fn optimize_local_seo(
        &self,
        business_name: &str,
        location: Location,
    ) -> Result<LocalSEOOptimization, String> {
        info!("Optimizing local SEO for: {}", business_name);

        let gmb = self.analyze_gmb_profile(business_name).await?;
        let citations = self.find_citations(business_name, &location).await?;
        let local_score = self.calculate_local_score(&gmb, &citations);
        let recommendations = self.generate_local_recommendations(&gmb, &citations, local_score);

        Ok(LocalSEOOptimization {
            business_name: business_name.to_string(),
            location,
            local_score,
            gmb_optimization: gmb,
            local_citations: citations,
            recommendations,
        })
    }

    /// Analyze Google My Business profile
    async fn analyze_gmb_profile(&self, _business_name: &str) -> Result<GMBOptimization, String> {
        // TODO: Integrate with Google My Business API
        Ok(GMBOptimization {
            profile_completeness: 85.0,
            review_count: 42,
            average_rating: 4.5,
            response_rate: 75.0,
            post_frequency: 2.5, // posts per week
        })
    }

    /// Find local citations
    async fn find_citations(&self, business_name: &str, location: &Location) -> Result<Vec<Citation>, String> {
        // TODO: Check citation sources (Yelp, Yellow Pages, etc.)
        Ok(vec![
            Citation {
                platform: "Yelp".to_string(),
                url: format!("https://yelp.com/biz/{}", business_name.to_lowercase().replace(' ', "-")),
                nap_consistency: true,
            },
            Citation {
                platform: "Yellow Pages".to_string(),
                url: format!("https://yellowpages.com/{}/{}", location.city, business_name.to_lowercase().replace(' ', "-")),
                nap_consistency: true,
            },
        ])
    }

    /// Calculate local SEO score
    fn calculate_local_score(&self, gmb: &GMBOptimization, citations: &[Citation]) -> f64 {
        let mut score: f64 = 0.0;

        // GMB profile (50%)
        score += (gmb.profile_completeness / 100.0) * 20.0;
        score += if gmb.review_count >= 25 { 15.0 } else { (gmb.review_count as f64 / 25.0) * 15.0 };
        score += if gmb.average_rating >= 4.0 { 15.0 } else { (gmb.average_rating / 5.0) * 15.0 };

        // Citations (30%)
        let consistent_citations = citations.iter().filter(|c| c.nap_consistency).count();
        score += (consistent_citations as f64 / citations.len().max(1) as f64) * 30.0;

        // Engagement (20%)
        score += (gmb.response_rate / 100.0) * 10.0;
        score += if gmb.post_frequency >= 2.0 { 10.0 } else { (gmb.post_frequency / 2.0) * 10.0 };

        score.min(100.0)
    }

    /// Generate local SEO recommendations
    fn generate_local_recommendations(
        &self,
        gmb: &GMBOptimization,
        citations: &[Citation],
        score: f64,
    ) -> Vec<String> {
        let mut recommendations = vec![];

        if gmb.profile_completeness < 100.0 {
            recommendations.push("Complete all sections of Google Business Profile".to_string());
        }

        if gmb.review_count < 50 {
            recommendations.push("Actively request reviews from satisfied customers".to_string());
        }

        if gmb.response_rate < 90.0 {
            recommendations.push("Respond to all customer reviews within 24 hours".to_string());
        }

        if gmb.post_frequency < 3.0 {
            recommendations.push("Post to Google Business Profile at least 3 times per week".to_string());
        }

        let inconsistent = citations.iter().filter(|c| !c.nap_consistency).count();
        if inconsistent > 0 {
            recommendations.push(format!("Fix NAP inconsistencies in {} citations", inconsistent));
        }

        if citations.len() < 10 {
            recommendations.push("Build citations on major local directories (Yelp, Bing Places, Apple Maps)".to_string());
        }

        if score > 80.0 {
            recommendations.push("Excellent local SEO! Focus on maintaining and growing reviews".to_string());
        }

        recommendations
    }

    /// Generate local schema markup
    pub fn generate_local_schema(&self, business: &str, location: &Location) -> String {
        format!(
            r#"{{
  "@context": "https://schema.org",
  "@type": "LocalBusiness",
  "name": "{}",
  "address": {{
    "@type": "PostalAddress",
    "streetAddress": "{}",
    "addressLocality": "{}",
    "addressRegion": "{}",
    "postalCode": "{}",
    "addressCountry": "{}"
  }}
}}"#,
            business,
            location.address,
            location.city,
            location.state,
            location.zip,
            location.country
        )
    }
}

impl Default for LocalSEOOptimizer {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_local_seo() {
        let optimizer = LocalSEOOptimizer;
        let location = Location {
            address: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            zip: "62701".to_string(),
            country: "USA".to_string(),
            latitude: None,
            longitude: None,
        };

        let result = optimizer.optimize_local_seo("Test Business", location).await;
        assert!(result.is_ok());
        
        let opt = result.unwrap();
        assert!(opt.local_score >= 0.0);
        assert!(!opt.recommendations.is_empty());
    }

    #[test]
    fn test_generate_local_schema() {
        let optimizer = LocalSEOOptimizer;
        let location = Location {
            address: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            zip: "62701".to_string(),
            country: "USA".to_string(),
            latitude: None,
            longitude: None,
        };

        let schema = optimizer.generate_local_schema("Test Business", &location);
        assert!(schema.contains("LocalBusiness"));
        assert!(schema.contains("Springfield"));
    }
}
