-- Add enterprise features: multi-tenancy, OAuth, analytics

CREATE TABLE IF NOT EXISTS tenants (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    custom_domain VARCHAR(255),
    plan VARCHAR(50) DEFAULT 'free', -- free, pro, enterprise,
    is_active BOOLEAN DEFAULT TRUE,
    settings JSON, -- Tenant-specific configuration,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subdomain ON tenants(subdomain);
CREATE INDEX IF NOT EXISTS idx_custom_domain ON tenants(custom_domain);

CREATE TABLE IF NOT EXISTS oauth_providers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL, -- google, github, microsoft,
    client_id VARCHAR(255) NOT NULL,
    client_secret VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_oauth_connections (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    provider_id INT NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (provider_id) REFERENCES oauth_providers(id) ON DELETE CASCADE,
    UNIQUE (provider_id, provider_user_id)
);

CREATE INDEX IF NOT EXISTS idx_user ON user_oauth_connections(user_id);

CREATE TABLE IF NOT EXISTS analytics_events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL, -- page_view, click, conversion,
    page_uuid VARCHAR(255),
    user_id INT,
    session_id VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    referer TEXT,
    metadata JSON, -- Additional event data,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_event_type ON analytics_events(event_type);
CREATE INDEX IF NOT EXISTS idx_page ON analytics_events(page_uuid);
CREATE INDEX IF NOT EXISTS idx_created ON analytics_events(created_at);
CREATE INDEX IF NOT EXISTS idx_session ON analytics_events(session_id);

CREATE TABLE IF NOT EXISTS backups (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    type VARCHAR(50) NOT NULL, -- full, incremental,
    status VARCHAR(50) DEFAULT 'pending', -- pending, completed, failed,
    file_path VARCHAR(500),
    file_size BIGINT, -- Size in bytes
    storage_location VARCHAR(100), -- s3, local, gcs,
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_status ON backups(status);
CREATE INDEX IF NOT EXISTS idx_created ON backups(created_at);

-- Add tenant_id to existing tables (makes them multi-tenant aware)
-- Note: Run these ALTER statements carefully in production
-- ALTER TABLE pages ADD COLUMN tenant_id INT;
-- ALTER TABLE users ADD COLUMN tenant_id INT;
-- ALTER TABLE media ADD COLUMN tenant_id INT;
-- ALTER TABLE modules ADD COLUMN tenant_id INT;

-- Add foreign keys (after adding tenant_id columns)
-- ALTER TABLE pages ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE users ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE media ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE modules ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
