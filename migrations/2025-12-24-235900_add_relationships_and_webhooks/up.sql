-- Add Content Relationships and Webhooks

-- Content Relationships Table
CREATE TABLE content_relationships (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Webhooks Table
CREATE TABLE webhooks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    url VARCHAR(500) NOT NULL,
    events JSON NOT NULL COMMENT 'Array of event types',
    secret VARCHAR(255) COMMENT 'HMAC secret for signatures',
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_triggered_at TIMESTAMP NULL,
    failure_count INT DEFAULT 0,
    INDEX idx_active (active)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Webhook Logs Table
CREATE TABLE webhook_logs (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    webhook_id INT NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSON,
    response_status INT,
    response_body TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (webhook_id) REFERENCES webhooks(id) ON DELETE CASCADE,
    INDEX idx_webhook (webhook_id),
    INDEX idx_created (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
