-- Create robots_rules table for configurable robots.txt

CREATE TABLE IF NOT EXISTS robots_rules (
    id SERIAL PRIMARY KEY,
    user_agent VARCHAR(100) NOT NULL DEFAULT '*',
    directive VARCHAR(20) NOT NULL CHECK (directive IN ('Allow', 'Disallow')),
    path VARCHAR(500) NOT NULL,
    crawl_delay INT DEFAULT NULL,
    comment VARCHAR(200),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_user_agent ON robots_rules(user_agent);
CREATE INDEX IF NOT EXISTS idx_active ON robots_rules(is_active);

-- Insert default rules
INSERT INTO robots_rules (user_agent, directive, path, comment, is_active) VALUES
('*', 'Disallow', '/v1/', 'Block API endpoints', TRUE),
('*', 'Disallow', '/admin/', 'Block admin area', TRUE),
('*', 'Allow', '/', 'Allow all other pages', TRUE);
