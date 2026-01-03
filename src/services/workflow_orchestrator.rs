use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::info;
use serde::{Deserialize, Serialize};

/// Workflow Orchestrator
/// Manages multi-step AI content generation workflows
pub struct WorkflowOrchestrator {
    workflows: Arc<Mutex<HashMap<String, Workflow>>>,
}

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub current_step: usize,
    pub status: WorkflowStatus,
    pub context: HashMap<String, String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub step_type: StepType,
    pub config: StepConfig,
    pub status: StepStatus,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Step types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepType {
    Research,           // Research topic
    GenerateDraft,      // Create initial content
    OptimizeSEO,        // Apply SEO optimization
    TranslateContent,   // Translate to another language
    ReviewContent,      // Quality review
    PublishContent,     // Publish final content
}

/// Step configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepConfig {
    pub provider: String,      // AI provider to use
    pub model: String,          // Model to use
    pub prompt_template: String,
    pub parameters: HashMap<String, String>,
}

/// Step status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Workflow status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Created,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl WorkflowOrchestrator {
    /// Create new orchestrator
    pub fn new() -> Self {
        info!("✅ Workflow Orchestrator initialized");
        Self {
            workflows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new workflow
    pub fn create_workflow(
        &self,
        name: String,
        steps: Vec<WorkflowStep>,
    ) -> Result<String, String> {
        let workflow_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();

        let workflow = Workflow {
            id: workflow_id.clone(),
            name,
            steps,
            current_step: 0,
            status: WorkflowStatus::Created,
            context: HashMap::new(),
            created_at: now,
            updated_at: now,
        };

        let mut workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        workflows.insert(workflow_id.clone(), workflow);
        
        info!("Created workflow: {}", workflow_id);
        Ok(workflow_id)
    }

    /// Start workflow execution
    pub fn start_workflow(&self, workflow_id: &str) -> Result<(), String> {
        let mut workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

        if workflow.status != WorkflowStatus::Created {
            return Err(format!("Workflow already started: {}", workflow_id));
        }

        workflow.status = WorkflowStatus::Running;
        workflow.updated_at = chrono::Utc::now().naive_utc();
        
        info!("Started workflow: {}", workflow_id);
        Ok(())
    }

    /// Execute next step in workflow
    pub async fn execute_next_step(&self, workflow_id: &str) -> Result<bool, String> {
        // Get current step info
        let (step_index, step) = {
            let workflows = self.workflows.lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            
            let workflow = workflows.get(workflow_id)
                .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

            if workflow.current_step >= workflow.steps.len() {
                return Ok(false); // No more steps
            }

            (workflow.current_step, workflow.steps[workflow.current_step].clone())
        };

        info!("Executing step {} of workflow {}", step_index, workflow_id);

        // Execute step (placeholder - in real implementation would call AI services)
        let result = self.execute_step(&step).await?;

        // Update step result
        {
            let mut workflows = self.workflows.lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            
            let workflow = workflows.get_mut(workflow_id)
                .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

            workflow.steps[step_index].status = StepStatus::Completed;
            workflow.steps[step_index].result = Some(result);
            workflow.current_step += 1;
            workflow.updated_at = chrono::Utc::now().naive_utc();

            // Check if workflow is complete
            if workflow.current_step >= workflow.steps.len() {
                workflow.status = WorkflowStatus::Completed;
                info!("Workflow completed: {}", workflow_id);
            }
        }

        Ok(true)
    }

    /// Execute a single step (placeholder)
    async fn execute_step(&self, step: &WorkflowStep) -> Result<String, String> {
        // Simulate step execution
        info!("Executing step: {} ({})", step.name, step.step_type.as_str());
        
        // In real implementation, this would:
        // 1. Get AI provider from step.config
        // 2. Build prompt from template + context
        // 3. Call AI service
        // 4. Process and return result
        
        Ok(format!("Result from step: {}", step.name))
    }

    /// Get workflow status
    pub fn get_workflow(&self, workflow_id: &str) -> Result<Workflow, String> {
        let workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        workflows.get(workflow_id)
            .cloned()
            .ok_or_else(|| format!("Workflow not found: {}", workflow_id))
    }

    /// Pause workflow
    pub fn pause_workflow(&self, workflow_id: &str) -> Result<(), String> {
        let mut workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

        if workflow.status == WorkflowStatus::Running {
            workflow.status = WorkflowStatus::Paused;
            workflow.updated_at = chrono::Utc::now().naive_utc();
            info!("Paused workflow: {}", workflow_id);
        }

        Ok(())
    }

    /// Resume workflow
    pub fn resume_workflow(&self, workflow_id: &str) -> Result<(), String> {
        let mut workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

        if workflow.status == WorkflowStatus::Paused {
            workflow.status = WorkflowStatus::Running;
            workflow.updated_at = chrono::Utc::now().naive_utc();
            info!("Resumed workflow: {}", workflow_id);
        }

        Ok(())
    }

    /// Cancel workflow
    pub fn cancel_workflow(&self, workflow_id: &str) -> Result<(), String> {
        let mut workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| format!("Workflow not found: {}", workflow_id))?;

        workflow.status = WorkflowStatus::Cancelled;
        workflow.updated_at = chrono::Utc::now().naive_utc();
        
        info!("Cancelled workflow: {}", workflow_id);
        Ok(())
    }

    /// List all workflows
    pub fn list_workflows(&self) -> Result<Vec<Workflow>, String> {
        let workflows = self.workflows.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        
        Ok(workflows.values().cloned().collect())
    }
}

impl StepType {
    pub fn as_str(&self) -> &str {
        match self {
            StepType::Research => "research",
            StepType::GenerateDraft => "generate_draft",
            StepType::OptimizeSEO => "optimize_seo",
            StepType::TranslateContent => "translate",
            StepType::ReviewContent => "review",
            StepType::PublishContent => "publish",
        }
    }
}

impl Default for WorkflowOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_workflow() {
        let orchestrator = WorkflowOrchestrator::new();
        
        let steps = vec![
            WorkflowStep {
                id: "1".to_string(),
                name: "Research".to_string(),
                step_type: StepType::Research,
                config: StepConfig {
                    provider: "openai".to_string(),
                    model: "gpt-4".to_string(),
                    prompt_template: "Research {topic}".to_string(),
                    parameters: HashMap::new(),
                },
                status: StepStatus::Pending,
                result: None,
                error: None,
            },
        ];

        let workflow_id = orchestrator.create_workflow("Test Workflow".to_string(), steps).unwrap();
        assert!(!workflow_id.is_empty());

        let workflow = orchestrator.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.status, WorkflowStatus::Created);
    }

    #[test]
    fn test_workflow_lifecycle() {
        let orchestrator = WorkflowOrchestrator::new();
        
        let steps = vec![];
        let workflow_id = orchestrator.create_workflow("Test".to_string(), steps).unwrap();

        // Start
        orchestrator.start_workflow(&workflow_id).unwrap();
        let workflow = orchestrator.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.status, WorkflowStatus::Running);

        // Pause
        orchestrator.pause_workflow(&workflow_id).unwrap();
        let workflow = orchestrator.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.status, WorkflowStatus::Paused);

        // Resume
        orchestrator.resume_workflow(&workflow_id).unwrap();
        let workflow = orchestrator.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.status, WorkflowStatus::Running);

        // Cancel
        orchestrator.cancel_workflow(&workflow_id).unwrap();
        let workflow = orchestrator.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.status, WorkflowStatus::Cancelled);
    }
}
