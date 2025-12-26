-- AI Provider Configurations
CREATE TABLE IF NOT EXISTS ai_provider_configs (
    id SERIAL PRIMARY KEY,
    provider_type VARCHAR(50) NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    model_name VARCHAR(100),
    is_active BOOLEAN DEFAULT true,
    daily_token_limit INTEGER,
    monthly_budget_cents INTEGER,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ai_providers_active ON ai_provider_configs(is_active);
CREATE INDEX IF NOT EXISTS idx_ai_providers_type ON ai_provider_configs(provider_type);
