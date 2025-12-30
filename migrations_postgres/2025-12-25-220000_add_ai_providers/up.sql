-- AI Provider Configuration System
-- Supports OpenAI, Anthropic, GCP Vertex AI, Azure OpenAI
-- with custom endpoint override for enterprise deployments

CREATE TABLE ai_provider_configs (
    id BIGSERIAL PRIMARY KEY,
    provider_type VARCHAR(20) NOT NULL, -- 'openai', 'anthropic', 'gcp', 'azure', 'custom'
    name VARCHAR(100) NOT NULL,
    
    -- Encrypted credentials (using app encryption key)
    api_key_encrypted BYTEA,
    
    -- Provider-specific config (JSON)
    -- OpenAI: {model, endpoint}
    -- Anthropic: {model, endpoint}
    -- GCP: {project_id, location, endpoint}
    -- Azure: {deployment, api_version, endpoint}
    config JSONB NOT NULL DEFAULT '{}',
    
    is_active BOOLEAN DEFAULT TRUE,
    is_default BOOLEAN DEFAULT FALSE,
    priority INTEGER DEFAULT 100, -- For fallback ordering (lower = higher priority)
    
    -- Budget controls
    daily_token_limit INTEGER,
    monthly_budget_cents INTEGER,
    
    -- Usage tracking
    tokens_used_today INTEGER DEFAULT 0,
    tokens_used_month INTEGER DEFAULT 0,
    cost_month_cents INTEGER DEFAULT 0,
    last_used_at TIMESTAMP,
    last_reset_daily TIMESTAMP DEFAULT NOW(),
    last_reset_monthly TIMESTAMP DEFAULT NOW(),
    
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id)
);

CREATE INDEX idx_ai_provider_active ON ai_provider_configs(is_active);
CREATE INDEX idx_ai_provider_default ON ai_provider_configs(is_default);
CREATE INDEX idx_ai_provider_priority ON ai_provider_configs(priority);

-- AI Usage Log for tracking and billing
CREATE TABLE ai_usage_log (
    id BIGSERIAL PRIMARY KEY,
    provider_id BIGINT REFERENCES ai_provider_configs(id),
    user_id INTEGER REFERENCES users(id),
    operation VARCHAR(50), -- 'generate', 'summarize', 'search', 'embedding', etc
    prompt_tokens INTEGER,
    completion_tokens INTEGER,
    total_tokens INTEGER,
    cost_cents INTEGER,
    model VARCHAR(50),
    latency_ms INTEGER,
    success BOOLEAN,
    error TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_ai_usage_provider ON ai_usage_log(provider_id);
CREATE INDEX idx_ai_usage_user ON ai_usage_log(user_id);
CREATE INDEX idx_ai_usage_created ON ai_usage_log(created_at);

-- AI Generated Content Tracking
CREATE TABLE ai_generated_content (
    id BIGSERIAL PRIMARY KEY,
    page_id VARCHAR(255) REFERENCES pages(uuid),
    provider_id BIGINT REFERENCES ai_provider_configs(id),
    content_type VARCHAR(50), -- 'title', 'meta', 'body', 'summary', 'keywords', 'tags'
    prompt TEXT,
    generated_content TEXT,
    model VARCHAR(50),
    tokens_used INTEGER,
    approved BOOLEAN DEFAULT FALSE,
    approved_by INTEGER REFERENCES users(id),
    approved_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_ai_content_page ON ai_generated_content(page_id);
CREATE INDEX idx_ai_content_type ON ai_generated_content(content_type);
CREATE INDEX idx_ai_content_approved ON ai_generated_content(approved);

-- AI Generation Queue for async processing
CREATE TABLE ai_generation_queue (
    id BIGSERIAL PRIMARY KEY,
    page_id VARCHAR(255),
    task_type VARCHAR(50), -- 'generate_meta', 'extract_keywords', 'generate_summary', etc
    priority INTEGER DEFAULT 5, -- 1 (highest) to 10 (lowest)
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    provider_id BIGINT REFERENCES ai_provider_configs(id),
    input_data JSONB,
    result_data JSONB,
    error TEXT,
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    created_at TIMESTAMP DEFAULT NOW(),
    started_at TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX idx_ai_queue_status ON ai_generation_queue(status);
CREATE INDEX idx_ai_queue_priority ON ai_generation_queue(priority);
CREATE INDEX idx_ai_queue_created ON ai_generation_queue(created_at);
