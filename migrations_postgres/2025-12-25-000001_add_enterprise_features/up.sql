-- Add enterprise features: multi-tenancy, OAuth, analytics

CREATE TABLE tenants (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    custom_domain VARCHAR(255),
    plan VARCHAR(50) DEFAULT 'free',
    is_active BOOLEAN DEFAULT TRUE,
    settings JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_subdomain ON tenants (subdomain);
CREATE INDEX idx_custom_domain ON tenants (custom_domain);

SELECT diesel_manage_updated_at('tenants');

CREATE TABLE oauth_providers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    client_secret VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('oauth_providers');

CREATE TABLE user_oauth_connections (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider_id INTEGER NOT NULL REFERENCES oauth_providers(id) ON DELETE CASCADE,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_provider_user UNIQUE (provider_id, provider_user_id)
);

CREATE INDEX idx_uoc_user ON user_oauth_connections (user_id);

SELECT diesel_manage_updated_at('user_oauth_connections');

CREATE TABLE analytics_events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    page_uuid VARCHAR(255),
    user_id INTEGER,
    session_id VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    referer TEXT,
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ae_event_type ON analytics_events (event_type);
CREATE INDEX idx_ae_page ON analytics_events (page_uuid);
CREATE INDEX idx_ae_created ON analytics_events (created_at);
CREATE INDEX idx_ae_session ON analytics_events (session_id);

CREATE TABLE backups (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    type VARCHAR(50) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    file_path VARCHAR(500),
    file_size BIGINT,
    storage_location VARCHAR(100),
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX idx_backup_status ON backups (status);
CREATE INDEX idx_backup_created ON backups (created_at);

-- Add tenant_id to existing tables (makes them multi-tenant aware)
-- Note: Run these ALTER statements carefully in production
-- ALTER TABLE pages ADD COLUMN tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE users ADD COLUMN tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE media ADD COLUMN tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE modules ADD COLUMN tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
-- Commented out as they might be dangerous on big tables, or require explicit migration.
-- Assuming schema.rs has them, we should probably add them if they are missing.
-- But schema.rs is our guide. Lines 680, 885 etc show tenant_id.
-- I'll uncomment them to match schema.

ALTER TABLE pages ADD COLUMN IF NOT EXISTS tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE media ADD COLUMN IF NOT EXISTS tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE modules ADD COLUMN IF NOT EXISTS tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE;
