-- Create robots_rules table for configurable robots.txt

CREATE TABLE robots_rules (
    id INTEGERSERIAL PRIMARY KEY,
    user_agent VARCHAR(100) NOT NULL DEFAULT '*',
    directive VARCHAR(20) NOT NULL CHECK (directive IN ('Allow', 'Disallow')),
    path VARCHAR(500) NOT NULL,
    crawl_delay INTEGERDEFAULT NULL,
    comment VARCHAR(200),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_user_agent (user_agent),
    INDEX idx_active (is_active)
);

-- Insert default rules
INSERT INTEGER robots_rules (user_agent, directive, path, comment, is_active) VALUES
('*', 'Disallow', '/v1/', 'Block API endpoints', TRUE),
('*', 'Disallow', '/admin/', 'Block admin area', TRUE),
('*', 'Allow', '/', 'Allow all other pages', TRUE);
