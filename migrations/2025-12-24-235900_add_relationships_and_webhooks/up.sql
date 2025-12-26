-- Add Content Relationships and Webhooks

-- Content Relationships Table
CREATE TABLE IF NOT EXISTS content_relationships (
    id BIGSERIAL PRIMARY KEY,
    source_type VARCHAR(20) CHECK (source_type IN ('page', 'module', 'media')) NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    target_type VARCHAR(20) CHECK (source_type IN ('page', 'module', 'media')) NOT NULL,
    target_id VARCHAR(255) NOT NULL,
    relationship_type VARCHAR(50) DEFAULT 'related',
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (source_type, source_id, target_type, target_id, relationship_type)
);

CREATE INDEX IF NOT EXISTS idx_source ON content_relationships(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_target ON content_relationships(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_type ON content_relationships(relationship_type);

-- Webhooks Table
CREATE TABLE IF NOT EXISTS webhooks (
    id SERIAL PRIMARY KEY,
    url VARCHAR(500) NOT NULL,
    events JSON NOT NULL, -- Array of event types,
    secret VARCHAR(255), -- HMAC secret for signatures,
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_triggered_at TIMESTAMP NULL,
    failure_count INT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_active ON webhooks(active);

-- Webhook Logs Table
CREATE TABLE IF NOT EXISTS webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    webhook_id INT NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSON,
    response_status INT,
    response_body TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (webhook_id) REFERENCES webhooks(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_webhook ON webhook_logs(webhook_id);
CREATE INDEX IF NOT EXISTS idx_created ON webhook_logs(created_at);
