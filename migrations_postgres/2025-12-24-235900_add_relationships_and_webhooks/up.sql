-- Add Content Relationships and Webhooks

-- Content Relationships Table
CREATE TABLE content_relationships (
    id BIGSERIAL PRIMARY KEY,
    source_type VARCHAR(20) NOT NULL CHECK (source_type IN ('page', 'module', 'media')),
    source_id VARCHAR(255) NOT NULL,
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('page', 'module', 'media')),
    target_id VARCHAR(255) NOT NULL,
    relationship_type VARCHAR(50) DEFAULT 'related',
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_relationship UNIQUE (source_type, source_id, target_type, target_id, relationship_type)
);

CREATE INDEX idx_rel_source ON content_relationships (source_type, source_id);
CREATE INDEX idx_rel_target ON content_relationships (target_type, target_id);
CREATE INDEX idx_rel_type ON content_relationships (relationship_type);

-- Webhooks Table
CREATE TABLE webhooks (
    id SERIAL PRIMARY KEY,
    url VARCHAR(500) NOT NULL,
    events JSON NOT NULL,
    secret VARCHAR(255),
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_triggered_at TIMESTAMP NULL,
    failure_count INTEGER DEFAULT 0
);

CREATE INDEX idx_webhooks_active ON webhooks (active);

-- Webhook Logs Table
CREATE TABLE webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    webhook_id INTEGER NOT NULL REFERENCES webhooks(id) ON DELETE CASCADE,
    event_type VARCHAR(100) NOT NULL,
    payload JSON,
    response_status INTEGER,
    response_body TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_wl_webhook ON webhook_logs (webhook_id);
CREATE INDEX idx_wl_created ON webhook_logs (created_at);
