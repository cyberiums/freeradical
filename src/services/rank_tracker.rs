use serde::{Deserialize, Serialize};
use log::info;

/// Rank Tracker
/// Monitor keyword rankings across search engines
pub struct RankTracker;

/// Rank tracking result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingReport {
    pub keyword: String,
    pub url: String,
    pub rankings: Vec<RankingSnapshot>,
    pub trend: RankTrend,
    pub visibility_score: f64,
}

/// Single ranking snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingSnapshot {
    pub search_engine: String,
    pub position: Option<u32>,
    pub timestamp: i64,
    pub page_url: String,
}

/// Ranking trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankTrend {
    pub direction: TrendDirection,
    pub position_change: i32,
    pub velocity: f64, // Change per day
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Rising,
    Stable,
    Falling,
}

impl RankTracker {
    /// Track keyword ranking
    pub async fn track_keyword(
        &self,
        keyword: &str,
        url: &str,
    ) -> Result<RankingReport, String> {
        info!("Tracking keyword: {} for {}", keyword, url);

        let rankings = self.get_rankings(keyword, url).await?;
        let trend = self.calculate_trend(&rankings);
        let visibility = self.calculate_visibility(&rankings);

        Ok(RankingReport {
            keyword: keyword.to_string(),
            url: url.to_string(),
            rankings,
            trend,
            visibility_score: visibility,
        })
    }

    /// Get current rankings
    async fn get_rankings(&self, _keyword: &str, url: &str) -> Result<Vec<RankingSnapshot>, String> {
        // TODO: Integrate with rank tracking APIs (SEMrush, Ahrefs, etc.)
        let timestamp = chrono::Utc::now().timestamp();
        
        Ok(vec![
            RankingSnapshot {
                search_engine: "Google".to_string(),
                position: Some(15),
                timestamp,
                page_url: url.to_string(),
            },
            RankingSnapshot {
                search_engine: "Bing".to_string(),
                position: Some(8),
                timestamp,
                page_url: url.to_string(),
            },
        ])
    }

    /// Calculate ranking trend
    fn calculate_trend(&self, rankings: &[RankingSnapshot]) -> RankTrend {
        if rankings.len() < 2 {
            return RankTrend {
                direction: TrendDirection::Stable,
                position_change: 0,
                velocity: 0.0,
            };
        }

        // Compare first (latest) with last (oldest)
        let latest_pos = rankings[0].position.unwrap_or(100) as i32;
        let oldest_pos = rankings[rankings.len() - 1].position.unwrap_or(100) as i32;
        
        let change = oldest_pos - latest_pos; // Positive = improved
        let time_diff = (rankings[0].timestamp - rankings[rankings.len() - 1].timestamp) as f64 / 86400.0; // days
        let velocity = if time_diff > 0.0 { change as f64 / time_diff } else { 0.0 };

        let direction = if change > 3 {
            TrendDirection::Rising
        } else if change < -3 {
            TrendDirection::Falling
        } else {
            TrendDirection::Stable
        };

        RankTrend {
            direction,
            position_change: change,
            velocity,
        }
    }

    /// Calculate visibility score
    fn calculate_visibility(&self, rankings: &[RankingSnapshot]) -> f64 {
        if rankings.is_empty() {
            return 0.0;
        }

        let mut total_score: f64 = 0.0;
        
        for ranking in rankings {
            if let Some(pos) = ranking.position {
                // Score based on position (position 1 = 100, position 100 = 0)
                let position_score = ((101 -pos) as f64 / 100.0) * 100.0;
                total_score += position_score;
            }
        }

        total_score / rankings.len() as f64
    }

    /// Track multiple keywords
    pub async fn track_bulk(&self, keywords: Vec<String>, url: &str) -> Result<BulkRankingReport, String> {
        info!("Bulk tracking {} keywords", keywords.len());

        let mut reports = vec![];
        for keyword in &keywords {
            let report = self.track_keyword(keyword, url).await?;
            reports.push(report);
        }

        let avg_visibility = reports.iter().map(|r| r.visibility_score).sum::<f64>() / reports.len() as f64;
        let top_10 = reports.iter().filter(|r| {
            r.rankings.iter().any(|s| s.position.map_or(false, |p| p <= 10))
        }).count();

        Ok(BulkRankingReport {
            total_keywords: keywords.len(),
            reports,
            average_visibility: avg_visibility,
            top_10_count: top_10,
        })
    }
}

/// Bulk ranking report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRankingReport {
    pub total_keywords: usize,
    pub reports: Vec<RankingReport>,
    pub average_visibility: f64,
    pub top_10_count: usize,
}

impl Default for RankTracker {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_track_keyword() {
        let tracker = RankTracker;
        let result = tracker.track_keyword("rust seo", "https://example.com").await;

        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(!report.rankings.is_empty());
    }

    #[tokio::test]
    async fn test_track_bulk() {
        let tracker = RankTracker;
        let keywords = vec!["test1".to_string(), "test2".to_string()];
        let result = tracker.track_bulk(keywords, "https://example.com").await;

        assert!(result.is_ok());
        let bulk = result.unwrap();
        assert_eq!(bulk.total_keywords, 2);
    }

    #[test]
    fn test_calculate_visibility() {
        let tracker = RankTracker;
        let rankings = vec![
            RankingSnapshot {
                search_engine: "Google".to_string(),
                position: Some(1),
                timestamp: 0,
                page_url: "test".to_string(),
            },
        ];

        let score = tracker.calculate_visibility(&rankings);
        assert!(score > 90.0); // Position 1 should have high visibility
    }
}
