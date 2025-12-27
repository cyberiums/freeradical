use serde::{Deserialize, Serialize};
use log::info;

/// Performance Monitor
/// Track and optimize SEO performance metrics
pub struct PerformanceMonitor;

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub overall_score: f64,
    pub page_speed: PageSpeedMetrics,
    pub core_web_vitals: CoreWebVitals,
    pub seo_health: SEOHealth,
    pub recommendations: Vec<String>,
}

/// Page speed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSpeedMetrics {
    pub load_time: f64,
    pub time_to_first_byte: f64,
    pub first_contentful_paint: f64,
    pub score: f64,
}

/// Core Web Vitals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebVitals {
    pub lcp: f64, // Largest Contentful Paint
    pub fid: f64, // First Input Delay
    pub cls: f64, // Cumulative Layout Shift
    pub score: f64,
}

/// SEO health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEOHealth {
    pub indexed_pages: usize,
    pub crawl_errors: usize,
    pub mobile_usability_issues: usize,
    pub score: f64,
}

impl PerformanceMonitor {
    /// Get comprehensive performance metrics
    pub async fn get_metrics(&self, url: &str) -> Result<PerformanceMetrics, String> {
        info!("Fetching performance metrics for: {}", url);

        let page_speed = self.measure_page_speed(url).await?;
        let core_web_vitals = self.measure_core_web_vitals(url).await?;
        let seo_health = self.check_seo_health(url).await?;

        let overall_score = (page_speed.score + core_web_vitals.score + seo_health.score) / 3.0;
        let recommendations = self.generate_recommendations(&page_speed, &core_web_vitals, &seo_health);

        Ok(PerformanceMetrics {
            overall_score,
            page_speed,
            core_web_vitals,
            seo_health,
            recommendations,
        })
    }

    /// Measure page speed
    async fn measure_page_speed(&self, _url: &str) -> Result<PageSpeedMetrics, String> {
        // TODO: Integrate with PageSpeed Insights API
        let load_time = 2.3;
        let ttfb = 0.4;
        let fcp = 1.2;

        let mut score: f64 = 100.0;
        if load_time > 3.0 { score -= 30.0; } else if load_time > 2.0 { score -= 15.0; }
        if ttfb > 0.6 { score -= 20.0; }
        if fcp > 1.8 { score -= 20.0; }

        Ok(PageSpeedMetrics {
            load_time,
            time_to_first_byte: ttfb,
            first_contentful_paint: fcp,
            score: score.max(0.0),
        })
    }

    /// Measure Core Web Vitals
    async fn measure_core_web_vitals(&self, _url: &str) -> Result<CoreWebVitals, String> {
        // TODO: Integrate with CrUX API or real measurement
        let lcp = 2.1; // Target: < 2.5s
        let fid = 80.0; // Target: < 100ms
        let cls = 0.08; // Target: < 0.1

        let mut score: f64 = 100.0;
        if lcp > 2.5 { score -= 30.0; }
        if fid > 100.0 { score -= 30.0; }
        if cls > 0.1 { score -= 30.0; }

        Ok(CoreWebVitals {
            lcp,
            fid,
            cls,
            score: score.max(0.0),
        })
    }

    /// Check SEO health
    async fn check_seo_health(&self, _url: &str) -> Result<SEOHealth, String> {
        // TODO: Integrate with Google Search Console API
        let indexed_pages = 150;
        let crawl_errors = 3;
        let mobile_issues = 1;

        let mut score: f64 = 100.0;
        score -= (crawl_errors as f64 * 10.0).min(40.0);
        score -= (mobile_issues as f64 * 15.0).min(30.0);

        Ok(SEOHealth {
            indexed_pages,
            crawl_errors,
            mobile_usability_issues: mobile_issues,
            score: score.max(0.0),
        })
    }

    /// Generate recommendations
    fn generate_recommendations(
        &self,
        page_speed: &PageSpeedMetrics,
        cwv: &CoreWebVitals,
        seo: &SEOHealth,
    ) -> Vec<String> {
        let mut recommendations = vec![];

        // Page speed
        if page_speed.load_time > 3.0 {
            recommendations.push("Optimize images and enable compression to improve load time".to_string());
        }
        if page_speed.time_to_first_byte > 0.6 {
            recommendations.push("Improve server response time (upgrade hosting or use CDN)".to_string());
        }

        // Core Web Vitals
        if cwv.lcp > 2.5 {
            recommendations.push("Reduce Largest Contentful Paint by optimizing largest elements".to_string());
        }
        if cwv.fid > 100.0 {
            recommendations.push("Minimize JavaScript execution time to improve interactivity".to_string());
        }
        if cwv.cls > 0.1 {
            recommendations.push("Set explicit dimensions for images/videos to prevent layout shifts".to_string());
        }

        // SEO Health
        if seo.crawl_errors > 0 {
            recommendations.push(format!("Fix {} crawl errors in Search Console", seo.crawl_errors));
        }
        if seo.mobile_usability_issues > 0 {
            recommendations.push("Address mobile usability issues for better mobile rankings".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Excellent performance! Keep monitoring metrics regularly.".to_string());
        }

        recommendations
    }

    /// Track performance over time
    pub fn track_trend(&self, historical_metrics: Vec<PerformanceMetrics>) -> PerformanceTrend {
        let count = historical_metrics.len();
        if count < 2 {
            return PerformanceTrend {
                trend_direction: TrendDirection::Stable,
                improvement_rate: 0.0,
                average_score: historical_metrics.first().map(|m| m.overall_score).unwrap_or(0.0),
            };
        }

        let latest = &historical_metrics[count - 1];
        let previous = &historical_metrics[count - 2];
        
        let improvement_rate = latest.overall_score - previous.overall_score;
        let trend = if improvement_rate > 2.0 {
            TrendDirection::Improving
        } else if improvement_rate < -2.0 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        let average: f64 = historical_metrics.iter().map(|m| m.overall_score).sum::<f64>() / count as f64;

        PerformanceTrend {
            trend_direction: trend,
            improvement_rate,
            average_score: average,
        }
    }
}

/// Performance trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub trend_direction: TrendDirection,
    pub improvement_rate: f64,
    pub average_score: f64,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_metrics() {
        let monitor = PerformanceMonitor;
        let result = monitor.get_metrics("https://example.com").await;

        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(metrics.overall_score >= 0.0);
        assert!(metrics.overall_score <= 100.0);
    }

    #[test]
    fn test_track_trend() {
        let monitor = PerformanceMonitor;
        let metrics = vec![
            PerformanceMetrics {
                overall_score: 70.0,
                page_speed: PageSpeedMetrics { load_time: 2.0, time_to_first_byte: 0.5, first_contentful_paint: 1.0, score: 80.0 },
                core_web_vitals: CoreWebVitals { lcp: 2.0, fid: 90.0, cls: 0.05, score: 85.0 },
                seo_health: SEOHealth { indexed_pages: 100, crawl_errors: 0, mobile_usability_issues: 0, score: 100.0 },
                recommendations: vec![],
            },
            PerformanceMetrics {
                overall_score: 75.0,
                page_speed: PageSpeedMetrics { load_time: 2.0, time_to_first_byte: 0.5, first_contentful_paint: 1.0, score: 80.0 },
                core_web_vitals: CoreWebVitals { lcp: 2.0, fid: 90.0, cls: 0.05, score: 85.0 },
                seo_health: SEOHealth { indexed_pages: 100, crawl_errors: 0, mobile_usability_issues: 0, score: 100.0 },
                recommendations: vec![],
            },
        ];

        let trend = monitor.track_trend(metrics);
        assert_eq!(trend.improvement_rate, 5.0);
    }
}
