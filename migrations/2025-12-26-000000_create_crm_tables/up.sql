-- CRM Core Tables for FreeRadical CMS
-- Version: v1.3.0
-- Created: 2025-12-25

-- Customer aggregated profile (extends users table)
CREATE TABLE crm_customers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Lifecycle tracking
    lifecycle_stage VARCHAR(50) NOT NULL DEFAULT 'lead', -- lead, prospect, customer, champion, churned
    customer_since TIMESTAMP,
    last_purchase_date TIMESTAMP,
    
    -- RFM Scoring (Recency, Frequency, Monetary)
    rfm_recency_score INTEGER DEFAULT 0, -- 1-5, higher = more recent
    rfm_frequency_score INTEGER DEFAULT 0, -- 1-5, higher = more frequent
    rfm_monetary_score INTEGER DEFAULT 0, -- 1-5, higher = more valuable
    rfm_total_score INTEGER GENERATED ALWAYS AS (rfm_recency_score + rfm_frequency_score + rfm_monetary_score) STORED,
    
    -- Metrics
    total_orders INTEGER DEFAULT 0,
    total_revenue NUMERIC(12, 2) DEFAULT 0.00,
    average_order_value NUMERIC(12, 2) DEFAULT 0.00,
    customer_lifetime_value NUMERIC(12, 2) DEFAULT 0.00,
    
    -- Engagement
    last_interaction_date TIMESTAMP,
    interaction_count INTEGER DEFAULT 0,
    email_open_rate NUMERIC(5, 2) DEFAULT 0.00, -- percentage
    email_click_rate NUMERIC(5, 2) DEFAULT 0.00, -- percentage
    
    -- Health & Risk
    health_score INTEGER DEFAULT 50, -- 0-100
    churn_risk VARCHAR(20) DEFAULT 'low', -- low, medium, high
    
    -- Segmentation
    primary_segment_id INTEGER,
    tags TEXT[], -- Array of tags for flexible categorization
    
    -- Metadata
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id)
);

CREATE INDEX idx_crm_customers_user_id ON crm_customers(user_id);
CREATE INDEX idx_crm_customers_lifecycle ON crm_customers(lifecycle_stage);
CREATE INDEX idx_crm_customers_rfm ON crm_customers(rfm_total_score DESC);
CREATE INDEX idx_crm_customers_health ON crm_customers(health_score DESC);
CREATE INDEX idx_crm_customers_segment ON crm_customers(primary_segment_id);

-- Customer interactions timeline
CREATE TABLE crm_interactions (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    
    -- Interaction details
    interaction_type VARCHAR(50) NOT NULL, -- email, call, meeting, order, support_ticket, page_view
    interaction_channel VARCHAR(50), -- web, email, phone, in_person, social
    
    -- Content
    subject VARCHAR(255),
    description TEXT,
    outcome VARCHAR(100), -- success, failed, pending, no_response
    
    -- Associated entities
    order_id INTEGER REFERENCES orders(id),
    related_entity_type VARCHAR(50), -- product, campaign, support_ticket
    related_entity_id INTEGER,
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Full-text search
    search_vector tsvector
);

CREATE INDEX idx_crm_interactions_customer ON crm_interactions(customer_id);
CREATE INDEX idx_crm_interactions_type ON crm_interactions(interaction_type);
CREATE INDEX idx_crm_interactions_date ON crm_interactions(created_at DESC);
CREATE INDEX idx_crm_interactions_search ON crm_interactions USING GIN(search_vector);

-- Customer segments for targeting
CREATE TABLE crm_segments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Segment criteria (stored as JSON for flexibility)
    criteria JSONB NOT NULL, -- {"rfm_score": {"min": 10}, "total_orders": {"min": 5}}
    
    -- Segment type
    is_dynamic BOOLEAN DEFAULT true, -- recalculated on query vs static list
    
    -- Stats
    customer_count INTEGER DEFAULT 0,
    last_calculated_at TIMESTAMP,
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(name)
);

CREATE INDEX idx_crm_segments_name ON crm_segments(name);
CREATE INDEX idx_crm_segments_criteria ON crm_segments USING GIN(criteria);

-- Segment membership (for static segments)
CREATE TABLE crm_segment_members (
    segment_id INTEGER NOT NULL REFERENCES crm_segments(id) ON DELETE CASCADE,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    added_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (segment_id, customer_id)
);

CREATE INDEX idx_segment_members_customer ON crm_segment_members(customer_id);

-- Marketing campaigns
CREATE TABLE crm_campaigns (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    campaign_type VARCHAR(50) NOT NULL, -- email, sms, push, social
    status VARCHAR(20) DEFAULT 'draft', -- draft, scheduled, active, paused, completed
    
    -- Targeting
    segment_id INTEGER REFERENCES crm_segments(id),
    target_customer_count INTEGER DEFAULT 0,
    
    -- Content
    subject VARCHAR(255),
    content TEXT,
    template_id INTEGER,
    
    -- Scheduling
    scheduled_at TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    
    -- Performance metrics
    sent_count INTEGER DEFAULT 0,
    delivered_count INTEGER DEFAULT 0,
    opened_count INTEGER DEFAULT 0,
    clicked_count INTEGER DEFAULT 0,
    converted_count INTEGER DEFAULT 0,
    revenue_generated NUMERIC(12, 2) DEFAULT 0.00,
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crm_campaigns_status ON crm_campaigns(status);
CREATE INDEX idx_crm_campaigns_segment ON crm_campaigns(segment_id);
CREATE INDEX idx_crm_campaigns_scheduled ON crm_campaigns(scheduled_at);

-- Tasks and reminders
CREATE TABLE crm_tasks (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES crm_customers(id) ON DELETE CASCADE,
    
    -- Task details
    title VARCHAR(200) NOT NULL,
    description TEXT,
    task_type VARCHAR(50), -- call, email, meeting, follow_up, demo
    priority VARCHAR(20) DEFAULT 'medium', -- low, medium, high, urgent
    status VARCHAR(20) DEFAULT 'pending', -- pending, in_progress, completed, cancelled
    
    -- Scheduling
    due_date TIMESTAMP,
    completed_at TIMESTAMP,
    
    -- Assignment
    assigned_to INTEGER REFERENCES users(id),
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crm_tasks_customer ON crm_tasks(customer_id);
CREATE INDEX idx_crm_tasks_assigned ON crm_tasks(assigned_to);
CREATE INDEX idx_crm_tasks_due ON crm_tasks(due_date);
CREATE INDEX idx_crm_tasks_status ON crm_tasks(status);

-- Customer notes
CREATE TABLE crm_notes (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    
    -- Note content
    note_text TEXT NOT NULL,
    is_pinned BOOLEAN DEFAULT false,
    
    -- Metadata
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crm_notes_customer ON crm_notes(customer_id);
CREATE INDEX idx_crm_notes_pinned ON crm_notes(is_pinned) WHERE is_pinned = true;

-- Update trigger for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_crm_customers_updated_at BEFORE UPDATE ON crm_customers FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_crm_segments_updated_at BEFORE UPDATE ON crm_segments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_crm_campaigns_updated_at BEFORE UPDATE ON crm_campaigns FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_crm_tasks_updated_at BEFORE UPDATE ON crm_tasks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_crm_notes_updated_at BEFORE UPDATE ON crm_notes FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
