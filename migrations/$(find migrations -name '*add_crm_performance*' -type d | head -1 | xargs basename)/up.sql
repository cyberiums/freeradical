-- Add performance indexes for CRM tables
-- These indexes optimize common query patterns

-- Customer lookups by lifecycle stage
CREATE INDEX idx_crm_customers_lifecycle ON crm_customers(lifecycle_stage);

-- Customer lookups by health score (for at-risk queries)
CREATE INDEX idx_crm_customers_health_score ON crm_customers(health_score DESC NULLS LAST);

-- Customer lookups by user_id (primary lookup)
CREATE INDEX idx_crm_customers_user_id ON crm_customers(user_id);

-- Customer churn risk filtering
CREATE INDEX idx_crm_customers_churn_risk ON crm_customers(churn_risk) WHERE churn_risk IS NOT NULL;

-- Interaction timeline queries (most common pattern)
CREATE INDEX idx_crm_interactions_customer_timeline ON crm_interactions(customer_id, created_at DESC);

-- Interaction type filtering
CREATE INDEX idx_crm_interactions_type ON crm_interactions(interaction_type);

-- Task queries by customer and status
CREATE INDEX idx_crm_tasks_customer_status ON crm_tasks(customer_id, status) WHERE customer_id IS NOT NULL;

-- Task queries by assignee and due date
CREATE INDEX idx_crm_tasks_assigned_due ON crm_tasks(assigned_to, due_date) WHERE assigned_to IS NOT NULL;

-- Task status for dashboard views
CREATE INDEX idx_crm_tasks_status ON crm_tasks(status) WHERE status IS NOT NULL;

-- Segment member lookups (both directions)
CREATE INDEX idx_crm_segment_members_segment ON crm_segment_members(segment_id);
CREATE INDEX idx_crm_segment_members_customer ON crm_segment_members(customer_id);

-- Campaign by segment (for targeting)
CREATE INDEX idx_crm_campaigns_segment ON crm_campaigns(segment_id) WHERE segment_id IS NOT NULL;

-- Campaign status filtering
CREATE INDEX idx_crm_campaigns_status ON crm_campaigns(status) WHERE status IS NOT NULL;

-- Notes by customer (timeline view)
CREATE INDEX idx_crm_notes_customer ON crm_notes(customer_id, created_at DESC);

-- Pinned notes (quick access)
CREATE INDEX idx_crm_notes_pinned ON crm_notes(customer_id) WHERE is_pinned = true;
