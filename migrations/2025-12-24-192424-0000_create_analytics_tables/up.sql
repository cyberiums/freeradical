-- Create analytics tables for built-in analytics
-- Privacy-compliant: No PII, IP hashing

CREATE TABLE IF NOT EXISTS page_views (
    id BIGSERIAL PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    page_uuid VARCHAR(36),
    visitor_hash VARCHAR(64) NOT NULL, -- SHA256 hash of IP for privacy
    referrer VARCHAR(500),
    user_agent VARCHAR(500),
    viewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_page_url ON page_views(page_url);
CREATE INDEX IF NOT EXISTS idx_page_uuid ON page_views(page_uuid);
CREATE INDEX IF NOT EXISTS idx_viewed_at ON page_views(viewed_at);
CREATE INDEX IF NOT EXISTS idx_visitor_hash ON page_views(visitor_hash);

CREATE TABLE IF NOT EXISTS analytics_summary (
    id SERIAL PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    date DATE NOT NULL,
    view_count INT DEFAULT 0,
    unique_visitors INT DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (page_url, date)
);

CREATE INDEX IF NOT EXISTS idx_date ON analytics_summary(date);

-- Note: visitor_hash uses SHA256 for privacy compliance
-- No personally identifiable information stored
