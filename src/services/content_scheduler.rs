use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// Content Scheduler
/// Queue system for AI content generation tasks
pub struct ContentScheduler {
    queue: Arc<Mutex<VecDeque<ScheduledTask>>>,
    running_tasks: Arc<Mutex<HashMap<String, ScheduledTask>>>,
    completed_tasks: Arc<Mutex<Vec<ScheduledTask>>>,
}

/// Scheduled task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub payload: TaskPayload,
    pub status: TaskStatus,
    pub scheduled_at: chrono::NaiveDateTime,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub error_message: Option<String>,
}

/// Task types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskType {
    GenerateContent,
    OptimizeSEO,
    TranslateContent,
    AnalyzeContent,
    SummarizeContent,
}

/// Task priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Task payload (generic data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPayload {
    pub data: HashMap<String, String>,
}

/// Task status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl ContentScheduler {
    /// Create new scheduler
    pub fn new() -> Self {
        info!("✅ Content Scheduler initialized");
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Schedule a new task
    pub fn schedule_task(
        &self,
        task_type: TaskType,
        priority: TaskPriority,
        payload: TaskPayload,
        max_retries: u32,
    ) -> Result<String, String> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let priority_for_log = priority.clone();
        
        let task = ScheduledTask {
            id: task_id.clone(),
            task_type,
            priority,
            payload,
            status: TaskStatus::Queued,
            scheduled_at: chrono::Utc::now().naive_utc(),
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries,
            error_message: None,
        };

        let mut queue = self.queue.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        queue.push_back(task);
        
        // Sort by priority (higher priority first)
        let mut tasks: Vec<_> = queue.drain(..).collect();
        tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        *queue = tasks.into();

        info!("Scheduled task {} with priority {:?}", task_id, priority_for_log);
        Ok(task_id)
    }

    /// Get next task from queue
    pub async fn get_next_task(&self) -> Result<Option<ScheduledTask>, String> {
        let mut queue = self.queue.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        if let Some(mut task) = queue.pop_front() {
            task.status = TaskStatus::Running;
            task.started_at = Some(chrono::Utc::now().naive_utc());
            
            let task_id_for_log = task.id.clone();
            let mut running = self.running_tasks.lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            running.insert(task.id.clone(), task.clone());
            
            info!("Started task: {}", task_id_for_log);
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    /// Mark task as completed
    pub fn complete_task(&self, task_id: &str) -> Result<(), String> {
        let mut running = self.running_tasks.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        if let Some(mut task) = running.remove(task_id) {
            task.status = TaskStatus::Completed;
            task.completed_at = Some(chrono::Utc::now().naive_utc());
            
            let mut completed = self.completed_tasks.lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            completed.push(task);
            
            info!("Completed task: {}", task_id);
            Ok(())
        } else {
            Err(format!("Task not found: {}", task_id))
        }
    }

    /// Mark task as failed (with retry logic)
    pub fn fail_task(&self, task_id: &str, error: String) -> Result<bool, String> {
        let mut running = self.running_tasks.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        if let Some(mut task) = running.remove(task_id) {
            task.retry_count += 1;
            task.error_message = Some(error.clone());
            
            let retry_count = task.retry_count;
            let max_retries = task.max_retries;
            
            if retry_count < max_retries {
                // Retry: put back in queue
                task.status = TaskStatus::Queued;
                task.started_at = None;
                
                let mut queue = self.queue.lock()
                    .map_err(|e| format!("Lock error: {}", e))?;
                queue.push_back(task);
                
                warn!("Task {} failed, retrying ({}/{})", task_id, retry_count, max_retries);
                Ok(true) // Will retry
            } else {
                // Max retries reached
                task.status = TaskStatus::Failed;
                task.completed_at = Some(chrono::Utc::now().naive_utc());
                
                let mut completed = self.completed_tasks.lock()
                    .map_err(|e| format!("Lock error: {}", e))?;
                completed.push(task);
                
                warn!("Task {} failed permanently after {} retries", task_id, max_retries);
                Ok(false) // Won't retry
            }
        } else {
            Err(format!("Task not found: {}", task_id))
        }
    }

    /// Get queue status
    pub fn get_status(&self) -> Result<SchedulerStatus, String> {
        let queue = self.queue.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let running = self.running_tasks.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let completed = self.completed_tasks.lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        Ok(SchedulerStatus {
            queued: queue.len(),
            running: running.len(),
            completed: completed.len(),
            failed: completed.iter().filter(|t| t.status == TaskStatus::Failed).count(),
        })
    }

    /// Cancel a task
    pub fn cancel_task(&self, task_id: &str) -> Result<(), String> {
        // Check queue
        let mut queue = self.queue.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        let original_len = queue.len();
        queue.retain(|t| t.id != task_id);
        
        if queue.len() < original_len {
            info!("Cancelled queued task: {}", task_id);
            return Ok(());
        }

        // Check running
        let mut running = self.running_tasks.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        if let Some(mut task) = running.remove(task_id) {
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(chrono::Utc::now().naive_utc());
            
            let mut completed = self.completed_tasks.lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            completed.push(task);
            
            info!("Cancelled running task: {}", task_id);
            Ok(())
        } else {
            Err(format!("Task not found: {}", task_id))
        }
    }
}

/// Scheduler status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStatus {
    pub queued: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
}

impl Default for ContentScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_task() {
        let scheduler = ContentScheduler::new();
        
        let mut payload = HashMap::new();
        payload.insert("topic".to_string(), "AI".to_string());
        
        let task_id = scheduler.schedule_task(
            TaskType::GenerateContent,
            TaskPriority::High,
            TaskPayload { data: payload },
            3,
        ).unwrap();

        assert!(!task_id.is_empty());
    }

    #[tokio::test]
    async fn test_task_lifecycle() {
        let scheduler = ContentScheduler::new();
        
        // Schedule
        let task_id = scheduler.schedule_task(
            TaskType::GenerateContent,
            TaskPriority::Normal,
            TaskPayload { data: HashMap::new() },
            1,
        ).unwrap();

        // Get next
        let task = scheduler.get_next_task().await.unwrap();
        assert!(task.is_some());
        assert_eq!(task.unwrap().status, TaskStatus::Running);

        // Complete
        scheduler.complete_task(&task_id).unwrap();
        
        let status = scheduler.get_status().unwrap();
        assert_eq!(status.completed, 1);
    }

    #[test]
    fn test_priority_queue() {
        let scheduler = ContentScheduler::new();
        
        // Schedule tasks with different priorities
        scheduler.schedule_task(
            TaskType::GenerateContent,
            TaskPriority::Low,
            TaskPayload { data: HashMap::new() },
            1,
        ).unwrap();

        scheduler.schedule_task(
            TaskType::GenerateContent,
            TaskPriority::Critical,
            TaskPayload { data: HashMap::new() },
            1,
        ).unwrap();

        // Critical priority should be first
        let queue = scheduler.queue.lock().unwrap();
        assert_eq!(queue.front().unwrap().priority, TaskPriority::Critical);
    }
}
