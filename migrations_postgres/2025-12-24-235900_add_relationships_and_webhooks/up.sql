-- Add Content Relationships and Webhooks

-- Content Relationships Table
CREATE TABLE content_relationships (
    id BIGINTEGERSERIAL PRIMARY KEY,
    source_type ENUM('page', 'module', 'media') NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    target_type ENUM('page', 'module', 'media') NOT NULL,
    target_id VARCHAR(255) NOT NULL,
    relationship_type VARCHAR(50) DEFAULT 'related',
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_source (source_type, source_id),
    INDEX idx_target (target_type, target_id),
    INDEX idx_type (relationship_type),
    UNIQUE KEY unique_relationship (source_type, source_id, target_type, target_id, relationship_type)
)  ;

-- Webhooks Table
CREATE TABLE webhooks (
    id INTEGERSERIAL PRIMARY KEY,
    url VARCHAR(500) NOT NULL,
    events JSON NOT NULL ,
    secret VARCHAR(255) ,
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_triggered_at TIMESTAMP NULL,
    failure_count INTEGERDEFAULT 0,
    INDEX idx_active (active)
)  ;

-- Webhook Logs Table
CREATE TABLE webhook_logs (
    id BIGINTEGERSERIAL PRIMARY KEY,
    webhook_id INTEGERNOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSON,
    response_status INTEGER
    response_body TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (webhook_id) REFERENCES webhooks(id) ON DELETE CASCADE,
    INDEX idx_webhook (webhook_id),
    INDEX idx_created (created_at)
)  ;
