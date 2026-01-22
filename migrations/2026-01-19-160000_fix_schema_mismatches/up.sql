-- Fix schema mismatches for media and ai_provider_configs

DROP TABLE IF EXISTS media CASCADE;
DROP TABLE IF EXISTS media_variants CASCADE;

CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,
    width INTEGER,
    height INTEGER,
    alt_text VARCHAR(255),
    title VARCHAR(255),
    description TEXT,
    tags TEXT[],
    uploaded_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tenant_id INTEGER REFERENCES tenants(id)
);

CREATE INDEX idx_media_uuid ON media(uuid);
CREATE INDEX idx_media_uploaded_by ON media(uploaded_by);
CREATE INDEX idx_media_tenant ON media(tenant_id);


DROP TABLE IF EXISTS ai_provider_configs CASCADE;

CREATE TABLE ai_provider_configs (
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

CREATE INDEX idx_ai_providers_active ON ai_provider_configs(is_active);
CREATE INDEX idx_ai_providers_type ON ai_provider_configs(provider_type);
