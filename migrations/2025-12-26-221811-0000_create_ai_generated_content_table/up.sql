CREATE TABLE IF NOT EXISTS ai_generated_content (
    id BIGSERIAL PRIMARY KEY,
    page_uuid VARCHAR(255),  -- Reference to pages.page_uuid (no FK constraint due to schema complexity)
    content_type VARCHAR(50) NOT NULL,  -- 'title', 'description', 'full_content', etc.
    prompt_used TEXT,
    generated_text TEXT NOT NULL,
    model_name VARCHAR(100),
    provider_type VARCHAR(50),  -- 'openai', 'anthropic', etc.
    tokens_used INTEGER,
    quality_score DECIMAL(3,2),  -- 0.00 to 1.00
    was_accepted BOOLEAN DEFAULT FALSE,
    generated_by INTEGER,  -- user_id
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ai_content_page ON ai_generated_content(page_uuid);
CREATE INDEX idx_ai_content_type ON ai_generated_content(content_type);
CREATE INDEX idx_ai_content_accepted ON ai_generated_content(was_accepted);
