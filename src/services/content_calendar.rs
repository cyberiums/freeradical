use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use log::info;
use std::collections::HashMap;

/// Content Calendar Planner
/// Strategic content planning and scheduling
pub struct ContentCalendar;

/// Content plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPlan {
    pub schedule: Vec<ContentItem>,
    pub coverage_score: f64,
    pub diversity_score: f64,
    pub recommendations: Vec<String>,
}

/// Scheduled content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: String,
    pub title: String,
    pub content_type: ContentType,
    pub target_keywords: Vec<String>,
    pub publish_date: DateTime<Utc>,
    pub status: PublishStatus,
    pub priority: Priority,
}

/// Content type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    BlogPost,
    Tutorial,
    CaseStudy,
    ProductReview,
    ListicleGuide,
    VideoScript,
}

/// Publish status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublishStatus {
    Planned,
    InProgress,
    Review,
    Scheduled,
    Published,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl ContentCalendar {
    /// Generate content plan for period
    pub async fn generate_plan(
        &self,
        start_date: DateTime<Utc>,
        days: u32,
        target_keywords: Vec<String>,
    ) -> Result<ContentPlan, String> {
        info!("Generating {} day content plan from {}", days, start_date);

        let schedule = self.create_schedule(start_date, days, &target_keywords).await?;
        let coverage_score = self.calculate_coverage(&schedule, &target_keywords);
        let diversity_score = self.calculate_diversity(&schedule);
        let recommendations = self.generate_recommendations(&schedule, coverage_score, diversity_score);

        Ok(ContentPlan {
            schedule,
            coverage_score,
            diversity_score,
            recommendations,
        })
    }

    /// Create content schedule
    async fn create_schedule(
        &self,
        start_date: DateTime<Utc>,
        days: u32,
        keywords: &[String],
    ) -> Result<Vec<ContentItem>, String> {
        let mut schedule = vec![];
        let posts_per_week = 3;
        let weeks = (days as f64 / 7.0).ceil() as u32;
        
        for week in 0..weeks {
            for post in 0..posts_per_week {
                let days_offset = (week *7) + (post * 2);
                if days_offset >= days {
                    break;
                }

                let publish_date = start_date + Duration::days(days_offset as i64);
                let keyword_idx = ((week * posts_per_week + post) as usize) % keywords.len();
                let keyword = keywords.get(keyword_idx).cloned().unwrap_or_else(|| "general".to_string());

                schedule.push(ContentItem {
                    id: format!("content-{}-{}", week, post),
                    title: format!("Comprehensive Guide to {}", keyword),
                    content_type: self.select_content_type(post),
                    target_keywords: vec![keyword],
                    publish_date,
                    status: PublishStatus::Planned,
                    priority: if post == 0 { Priority::High } else { Priority::Medium },
                });
            }
        }

        Ok(schedule)
    }

    /// Select content type for variety
    fn select_content_type(&self, index: u32) -> ContentType {
        match index % 6 {
            0 => ContentType::BlogPost,
            1 => ContentType::Tutorial,
            2 => ContentType::CaseStudy,
            3 => ContentType::ProductReview,
            4 => ContentType::ListicleGuide,
            _ => ContentType::VideoScript,
        }
    }

    /// Calculate keyword coverage
    fn calculate_coverage(&self, schedule: &[ContentItem], keywords: &[String]) -> f64 {
        let mut covered_keywords = std::collections::HashSet::new();
        
        for item in schedule {
            for keyword in &item.target_keywords {
                covered_keywords.insert(keyword.clone());
            }
        }

        if keywords.is_empty() {
            return 100.0;
        }

        (covered_keywords.len() as f64 / keywords.len() as f64) * 100.0
    }

    /// Calculate content diversity
    fn calculate_diversity(&self, schedule: &[ContentItem]) -> f64 {
        let mut type_counts: HashMap<String, u32> = HashMap::new();
        
        for item in schedule {
            let type_name = format!("{:?}", item.content_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }

        if schedule.is_empty() {
            return 0.0;
        }

        // Calculate entropy-based diversity (simplified)
        let total = schedule.len() as f64;
        let unique_types = type_counts.len() as f64;
        
        (unique_types / 6.0) * 100.0 // 6 is max content types
    }

    /// Generate recommendations
    fn generate_recommendations(
        &self,
        schedule: &[ContentItem],
        coverage: f64,
        diversity: f64,
    ) -> Vec<String> {
        let mut recommendations = vec![];

        if coverage < 80.0 {
            recommendations.push("Increase content to cover more target keywords".to_string());
        }

        if diversity < 50.0 {
            recommendations.push("Add more variety in content types".to_string());
        }

        if schedule.len() < 10 {
            recommendations.push("Consider publishing more frequently for better SEO momentum".to_string());
        }

        let high_priority = schedule.iter().filter(|i| matches!(i.priority, Priority::High)).count();
        if high_priority < schedule.len() / 4 {
            recommendations.push("Mark more strategic content as high priority".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Content plan looks well-balanced!".to_string());
        }

        recommendations
    }

    /// Optimize publishing schedule
    pub fn optimize_schedule(&self, mut items: Vec<ContentItem>) -> Vec<ContentItem> {
        // Sort by priority and date
        items.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (Priority::High, Priority::High) | 
                (Priority::Medium, Priority::Medium) | 
                (Priority::Low, Priority::Low) => a.publish_date.cmp(&b.publish_date),
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, Priority::Low) => std::cmp::Ordering::Less,
                (Priority::Low, Priority::Medium) => std::cmp::Ordering::Greater,
            }
        });

        items
    }
}

impl Default for ContentCalendar {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_plan() {
        let calendar = ContentCalendar;
        let start = Utc::now();
        let keywords = vec!["SEO".to_string(), "content".to_string()];
        
        let result = calendar.generate_plan(start, 30, keywords).await;
        assert!(result.is_ok());
        
        let plan = result.unwrap();
        assert!(!plan.schedule.is_empty());
        assert!(plan.coverage_score >= 0.0);
        assert!(plan.diversity_score >= 0.0);
    }

    #[test]
    fn test_optimize_schedule() {
        let calendar = ContentCalendar;
        let now = Utc::now();
        
        let items = vec![
            ContentItem {
                id: "1".to_string(),
                title: "Test".to_string(),
                content_type: ContentType::BlogPost,
                target_keywords: vec![],
                publish_date: now,
                status: PublishStatus::Planned,
                priority: Priority::Low,
            },
            ContentItem {
                id: "2".to_string(),
                title: "Test2".to_string(),
                content_type: ContentType::Tutorial,
                target_keywords: vec![],
                publish_date: now,
                status: PublishStatus::Planned,
                priority: Priority::High,
            },
        ];

        let optimized = calendar.optimize_schedule(items);
        assert_eq!(optimized[0].id, "2"); // High priority first
    }
}
