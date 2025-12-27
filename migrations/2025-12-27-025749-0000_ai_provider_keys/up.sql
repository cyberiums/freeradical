-- Create AI provider keys table for secure storage of API keys
CREATE TABLE ai_provider_keys (
    id SERIAL PRIMARY KEY,
    provider_name VARCHAR(50) NOT NULL,
    key_name VARCHAR(100) NOT NULL,
    encrypted_key TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rotated_at TIMESTAMP,
    last_used_at TIMESTAMP,
    request_count INTEGER NOT NULL DEFAULT 0,
    token_count BIGINT NOT NULL DEFAULT 0,
    notes TEXT,
    CONSTRAINT unique_provider_key UNIQUE (provider_name, key_name)
);

-- Create index for active keys lookup
CREATE INDEX idx_ai_provider_keys_active ON ai_provider_keys(provider_name, is_active);

-- Create index for usage tracking
CREATE INDEX idx_ai_provider_keys_last_used ON ai_provider_keys(last_used_at);

-- Create key rotation history table
CREATE TABLE ai_key_rotation_history (
    id SERIAL PRIMARY KEY,
    provider_key_id INTEGER NOT NULL REFERENCES ai_provider_keys(id) ON DELETE CASCADE,
    rotated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason VARCHAR(255),
    rotated_by INTEGER,
    old_key_hash VARCHAR(64)
);

-- Create index for rotation history lookup
CREATE INDEX idx_ai_key_rotation_history_provider ON ai_key_rotation_history(provider_key_id);

COMMENT ON TABLE ai_provider_keys IS 'Encrypted storage for AI provider API keys (OpenAI, Anthropic, Google, etc.)';
COMMENT ON COLUMN ai_provider_keys.encrypted_key IS 'AES-256-GCM encrypted API key using v1.6.0 encryption service';
COMMENT ON COLUMN ai_provider_keys.request_count IS 'Total number of API requests made with this key';
COMMENT ON COLUMN ai_provider_keys.token_count IS 'Total number of tokens consumed using this key';
