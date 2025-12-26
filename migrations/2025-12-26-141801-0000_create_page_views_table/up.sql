-- Create page_views table for analytics
CREATE TABLE page_views (
    id SERIAL PRIMARY KEY,
    page_id INTEGER NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    ip_address VARCHAR(45), -- IPv6 compatible
    user_agent TEXT,
    referer TEXT,
    country_code VARCHAR(2),
    city VARCHAR(100),
    viewed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    session_id VARCHAR(255),
    duration_seconds INTEGER,
    
    -- Indexes for common queries
    CONSTRAINT fk_page FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
);

CREATE INDEX idx_page_views_page ON page_views(page_id, viewed_at DESC);
CREATE INDEX idx_page_views_user ON page_views(user_id, viewed_at DESC) WHERE user_id IS NOT NULL;
CREATE INDEX idx_page_views_session ON page_views(session_id) WHERE session_id IS NOT NULL;
CREATE INDEX idx_page_views_date ON page_views(viewed_at DESC);
