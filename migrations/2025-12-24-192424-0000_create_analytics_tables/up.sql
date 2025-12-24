-- Create analytics tables for built-in analytics
-- Privacy-compliant: No PII, IP hashing

CREATE TABLE page_views (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    page_uuid VARCHAR(36),
    visitor_hash VARCHAR(64) NOT NULL COMMENT 'SHA256 hash of IP for privacy',
    referrer VARCHAR(500),
    user_agent VARCHAR(500),
    viewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_page_url (page_url),
    INDEX idx_page_uuid (page_uuid),
    INDEX idx_viewed_at (viewed_at),
    INDEX idx_visitor_hash (visitor_hash)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE analytics_summary (
    id INT AUTO_INCREMENT PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    date DATE NOT NULL,
    view_count INT DEFAULT 0,
    unique_visitors INT DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY idx_page_date (page_url, date),
    INDEX idx_date (date)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Note: visitor_hash uses SHA256 for privacy compliance
-- No personally identifiable information stored
