-- Create analytics tables for built-in analytics
-- Privacy-compliant: No PII, IP hashing

CREATE TABLE page_views (
    id BIGSERIAL PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    page_uuid VARCHAR(36),
    visitor_hash VARCHAR(64) NOT NULL,
    referrer VARCHAR(500),
    user_agent VARCHAR(500),
    viewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_pv_page_url ON page_views (page_url);
CREATE INDEX idx_pv_page_uuid ON page_views (page_uuid);
CREATE INDEX idx_pv_viewed_at ON page_views (viewed_at);
CREATE INDEX idx_pv_visitor_hash ON page_views (visitor_hash);

CREATE TABLE analytics_summary (
    id SERIAL PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    date DATE NOT NULL,
    view_count INTEGER DEFAULT 0,
    unique_visitors INTEGER DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_page_date UNIQUE (page_url, date)
);

CREATE INDEX idx_as_date ON analytics_summary (date);

SELECT diesel_manage_updated_at('analytics_summary');

-- Note: visitor_hash uses SHA256 for privacy compliance
-- No personally identifiable information stored
