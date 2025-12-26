-- Add performance indexes for CRM tables

-- Customer lookups
CREATE INDEX IF NOT EXISTS idx_crm_customers_lifecycle ON crm_customers(lifecycle_stage);
CREATE INDEX IF NOT EXISTS idx_crm_customers_health_score ON crm_customers(health_score DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS idx_crm_customers_user_id ON crm_customers(user_id);
CREATE INDEX IF NOT EXISTS idx_crm_customers_churn_risk ON crm_customers(churn_risk) WHERE churn_risk IS NOT NULL;

-- Interaction queries
CREATE INDEX IF NOT EXISTS idx_crm_interactions_customer_timeline ON crm_interactions(customer_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_type ON crm_interactions(interaction_type);

-- Task queries
CREATE INDEX IF NOT EXISTS idx_crm_tasks_customer_status ON crm_tasks(customer_id, status) WHERE customer_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_crm_tasks_assigned_due ON crm_tasks(assigned_to, due_date) WHERE assigned_to IS NOT NULL;

-- Segment members
CREATE INDEX IF NOT EXISTS idx_crm_segment_members_segment ON crm_segment_members(segment_id);
CREATE INDEX IF NOT EXISTS idx_crm_segment_members_customer ON crm_segment_members(customer_id);

-- Campaigns
CREATE INDEX IF NOT EXISTS idx_crm_campaigns_segment ON crm_campaigns(segment_id) WHERE segment_id IS NOT NULL;

-- Notes
CREATE INDEX IF NOT EXISTS idx_crm_notes_customer ON crm_notes(customer_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_crm_notes_pinned ON crm_notes(customer_id) WHERE is_pinned = true;
