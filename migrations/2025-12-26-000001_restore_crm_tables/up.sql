-- RESTORE CRM Tables (Missing in local DB but marked as run)

-- Customer aggregated profile (extends users table)
CREATE TABLE IF NOT EXISTS crm_customers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lifecycle_stage VARCHAR(50) NOT NULL DEFAULT 'lead',
    customer_since TIMESTAMP,
    last_purchase_date TIMESTAMP,
    rfm_recency_score INTEGER DEFAULT 0,
    rfm_frequency_score INTEGER DEFAULT 0,
    rfm_monetary_score INTEGER DEFAULT 0,
    rfm_total_score INTEGER GENERATED ALWAYS AS (rfm_recency_score + rfm_frequency_score + rfm_monetary_score) STORED,
    total_orders INTEGER DEFAULT 0,
    total_revenue NUMERIC(12, 2) DEFAULT 0.00,
    average_order_value NUMERIC(12, 2) DEFAULT 0.00,
    customer_lifetime_value NUMERIC(12, 2) DEFAULT 0.00,
    last_interaction_date TIMESTAMP,
    interaction_count INTEGER DEFAULT 0,
    email_open_rate NUMERIC(5, 2) DEFAULT 0.00,
    email_click_rate NUMERIC(5, 2) DEFAULT 0.00,
    health_score INTEGER DEFAULT 50,
    churn_risk VARCHAR(20) DEFAULT 'low',
    primary_segment_id INTEGER,
    tags TEXT[],
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id)
);

CREATE INDEX IF NOT EXISTS idx_crm_customers_user_id ON crm_customers(user_id);
CREATE INDEX IF NOT EXISTS idx_crm_customers_lifecycle ON crm_customers(lifecycle_stage);
CREATE INDEX IF NOT EXISTS idx_crm_customers_rfm ON crm_customers(rfm_total_score DESC);
CREATE INDEX IF NOT EXISTS idx_crm_customers_health ON crm_customers(health_score DESC);
CREATE INDEX IF NOT EXISTS idx_crm_customers_segment ON crm_customers(primary_segment_id);

-- Customer interactions timeline
CREATE TABLE IF NOT EXISTS crm_interactions (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    interaction_type VARCHAR(50) NOT NULL,
    interaction_channel VARCHAR(50),
    subject VARCHAR(255),
    description TEXT,
    outcome VARCHAR(100),
    order_id INTEGER REFERENCES orders(id),
    related_entity_type VARCHAR(50),
    related_entity_id INTEGER,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    search_vector tsvector
);

CREATE INDEX IF NOT EXISTS idx_crm_interactions_customer ON crm_interactions(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_type ON crm_interactions(interaction_type);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_date ON crm_interactions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_crm_interactions_search ON crm_interactions USING GIN(search_vector);

-- Customer segments for targeting
CREATE TABLE IF NOT EXISTS crm_segments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    criteria JSONB NOT NULL,
    is_dynamic BOOLEAN DEFAULT true,
    customer_count INTEGER DEFAULT 0,
    last_calculated_at TIMESTAMP,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name)
);

CREATE INDEX IF NOT EXISTS idx_crm_segments_name ON crm_segments(name);
CREATE INDEX IF NOT EXISTS idx_crm_segments_criteria ON crm_segments USING GIN(criteria);

-- Segment membership (for static segments)
CREATE TABLE IF NOT EXISTS crm_segment_members (
    segment_id INTEGER NOT NULL REFERENCES crm_segments(id) ON DELETE CASCADE,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    added_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (segment_id, customer_id)
);

CREATE INDEX IF NOT EXISTS idx_segment_members_customer ON crm_segment_members(customer_id);

-- Marketing campaigns
CREATE TABLE IF NOT EXISTS crm_campaigns (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    campaign_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'draft',
    segment_id INTEGER REFERENCES crm_segments(id),
    target_customer_count INTEGER DEFAULT 0,
    subject VARCHAR(255),
    content TEXT,
    template_id INTEGER,
    scheduled_at TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    sent_count INTEGER DEFAULT 0,
    delivered_count INTEGER DEFAULT 0,
    opened_count INTEGER DEFAULT 0,
    clicked_count INTEGER DEFAULT 0,
    converted_count INTEGER DEFAULT 0,
    revenue_generated NUMERIC(12, 2) DEFAULT 0.00,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_crm_campaigns_status ON crm_campaigns(status);
CREATE INDEX IF NOT EXISTS idx_crm_campaigns_segment ON crm_campaigns(segment_id);
CREATE INDEX IF NOT EXISTS idx_crm_campaigns_scheduled ON crm_campaigns(scheduled_at);

-- Tasks and reminders
CREATE TABLE IF NOT EXISTS crm_tasks (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES crm_customers(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    task_type VARCHAR(50),
    priority VARCHAR(20) DEFAULT 'medium',
    status VARCHAR(20) DEFAULT 'pending',
    due_date TIMESTAMP,
    completed_at TIMESTAMP,
    assigned_to INTEGER REFERENCES users(id),
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_crm_tasks_customer ON crm_tasks(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_tasks_assigned ON crm_tasks(assigned_to);
CREATE INDEX IF NOT EXISTS idx_crm_tasks_due ON crm_tasks(due_date);
CREATE INDEX IF NOT EXISTS idx_crm_tasks_status ON crm_tasks(status);

-- Customer notes
CREATE TABLE IF NOT EXISTS crm_notes (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES crm_customers(id) ON DELETE CASCADE,
    note_text TEXT NOT NULL,
    is_pinned BOOLEAN DEFAULT false,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_crm_notes_customer ON crm_notes(customer_id);
CREATE INDEX IF NOT EXISTS idx_crm_notes_pinned ON crm_notes(is_pinned) WHERE is_pinned = true;

-- Update trigger for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_crm_customers_updated_at ON crm_customers;
CREATE TRIGGER update_crm_customers_updated_at BEFORE UPDATE ON crm_customers FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_crm_segments_updated_at ON crm_segments;
CREATE TRIGGER update_crm_segments_updated_at BEFORE UPDATE ON crm_segments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_crm_campaigns_updated_at ON crm_campaigns;
CREATE TRIGGER update_crm_campaigns_updated_at BEFORE UPDATE ON crm_campaigns FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_crm_tasks_updated_at ON crm_tasks;
CREATE TRIGGER update_crm_tasks_updated_at BEFORE UPDATE ON crm_tasks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_crm_notes_updated_at ON crm_notes;
CREATE TRIGGER update_crm_notes_updated_at BEFORE UPDATE ON crm_notes FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
