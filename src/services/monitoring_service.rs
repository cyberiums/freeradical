// Performance Monitoring Service
// Tracks metrics for Iteration 4 features

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Iteration 4 Performance Metrics
pub struct PerformanceMetrics {
    // Media Library metrics
    pub media_uploads_total: AtomicU64,
    pub media_uploads_failed: AtomicU64,
    pub media_bytes_uploaded: AtomicU64,
    pub media_avg_upload_time_ms: AtomicU64,
    
    // Revision History metrics
    pub revisions_created: AtomicU64,
    pub rollbacks_performed: AtomicU64,
    pub rollbacks_failed: AtomicU64,
    
    // Scheduled Publishing metrics
    pub scheduler_runs: AtomicU64,
    pub pages_auto_published: AtomicU64,
    pub pages_auto_archived: AtomicU64,
    pub scheduler_errors: AtomicU64,
    
    // General metrics
    pub server_start_time: DateTime<Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            media_uploads_total: AtomicU64::new(0),
            media_uploads_failed: AtomicU64::new(0),
            media_bytes_uploaded: AtomicU64::new(0),
            media_avg_upload_time_ms: AtomicU64::new(0),
            revisions_created: AtomicU64::new(0),
            rollbacks_performed: AtomicU64::new(0),
            rollbacks_failed: AtomicU64::new(0),
            scheduler_runs: AtomicU64::new(0),
            pages_auto_published: AtomicU64::new(0),
            pages_auto_archived: AtomicU64::new(0),
            scheduler_errors: AtomicU64::new(0),
            server_start_time: Utc::now(),
        }
    }
}

impl PerformanceMetrics {
    /// Record a successful media upload
    pub fn record_upload(&self, bytes: u64, duration_ms: u64) {
        self.media_uploads_total.fetch_add(1, Ordering::Relaxed);
        self.media_bytes_uploaded.fetch_add(bytes, Ordering::Relaxed);
        
        // Simple moving average
        let current_avg = self.media_avg_upload_time_ms.load(Ordering::Relaxed);
        let total = self.media_uploads_total.load(Ordering::Relaxed);
        let new_avg = ((current_avg * (total - 1)) + duration_ms) / total;
        self.media_avg_upload_time_ms.store(new_avg, Ordering::Relaxed);
    }
    
    /// Record a failed media upload
    pub fn record_upload_failure(&self) {
        self.media_uploads_failed.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Record a revision creation
    pub fn record_revision(&self) {
        self.revisions_created.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Record a successful rollback
    pub fn record_rollback(&self, success: bool) {
        if success {
            self.rollbacks_performed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.rollbacks_failed.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Record scheduler activity
    pub fn record_scheduler_run(&self, published: u64, archived: u64, errors: u64) {
        self.scheduler_runs.fetch_add(1, Ordering::Relaxed);
        self.pages_auto_published.fetch_add(published, Ordering::Relaxed);
        self.pages_auto_archived.fetch_add(archived, Ordering::Relaxed);
        self.scheduler_errors.fetch_add(errors, Ordering::Relaxed);
    }
    
    /// Get metrics snapshot as JSON
    pub fn to_json(&self) -> serde_json::Value {
        let uptime_seconds = (Utc::now() - self.server_start_time).num_seconds();
        
        serde_json::json!({
            "server": {
                "start_time": self.server_start_time.to_rfc3339(),
                "uptime_seconds": uptime_seconds,
                "uptime_hours": uptime_seconds / 3600
            },
            "media_library": {
                "uploads_total": self.media_uploads_total.load(Ordering::Relaxed),
                "uploads_failed": self.media_uploads_failed.load(Ordering::Relaxed),
                "bytes_uploaded": self.media_bytes_uploaded.load(Ordering::Relaxed),
                "avg_upload_time_ms": self.media_avg_upload_time_ms.load(Ordering::Relaxed),
                "success_rate": self.calculate_success_rate(
                    self.media_uploads_total.load(Ordering::Relaxed),
                    self.media_uploads_failed.load(Ordering::Relaxed)
                )
            },
            "revision_history": {
                "revisions_created": self.revisions_created.load(Ordering::Relaxed),
                "rollbacks_performed": self.rollbacks_performed.load(Ordering::Relaxed),
                "rollbacks_failed": self.rollbacks_failed.load(Ordering::Relaxed),
                "rollback_success_rate": self.calculate_success_rate(
                    self.rollbacks_performed.load(Ordering::Relaxed),
                    self.rollbacks_failed.load(Ordering::Relaxed)
                )
            },
            "scheduled_publishing": {
                "scheduler_runs": self.scheduler_runs.load(Ordering::Relaxed),
                "pages_auto_published": self.pages_auto_published.load(Ordering::Relaxed),
                "pages_auto_archived": self.pages_auto_archived.load(Ordering::Relaxed),
                "scheduler_errors": self.scheduler_errors.load(Ordering::Relaxed),
                "error_rate": self.calculate_error_rate(
                    self.scheduler_runs.load(Ordering::Relaxed),
                    self.scheduler_errors.load(Ordering::Relaxed)
                )
            }
        })
    }
    
    fn calculate_success_rate(&self, total: u64, failed: u64) -> f64 {
        if total == 0 {
            return 100.0;
        }
        let successful = total.saturating_sub(failed);
        (successful as f64 / total as f64) * 100.0
    }
    
    fn calculate_error_rate(&self, total: u64, errors: u64) -> f64 {
        if total == 0 {
            return 0.0;
        }
        (errors as f64 / total as f64) * 100.0
    }
}

/// Global metrics instance
pub static METRICS: once_cell::sync::Lazy<Arc<PerformanceMetrics>> = 
    once_cell::sync::Lazy::new(|| Arc::new(PerformanceMetrics::default()));

/// Log current metrics to console
pub fn log_metrics() {
    let metrics = METRICS.to_json();
    log::info!("📊 Performance Metrics:\n{}", serde_json::to_string_pretty(&metrics).unwrap());
}
