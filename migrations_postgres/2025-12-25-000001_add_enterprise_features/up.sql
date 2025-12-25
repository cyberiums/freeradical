-- Add enterprise features: multi-tenancy, OAuth, analytics

CREATE TABLE tenants (
    id INTEGERPRIMARY KEY SERIAL,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    custom_domain VARCHAR(255),
    plan VARCHAR(50) DEFAULT 'free' ,
    is_active BOOLEAN DEFAULT TRUE,
    settings JSON ,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_subdomain (subdomain),
    INDEX idx_custom_domain (custom_domain)
);

CREATE TABLE oauth_providers (
    id INTEGERPRIMARY KEY SERIAL,
    name VARCHAR(100) NOT NULL ,
    client_id VARCHAR(255) NOT NULL,
    client_secret VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE user_oauth_connections (
    id INTEGERPRIMARY KEY SERIAL,
    user_id INTEGERNOT NULL,
    provider_id INTEGERNOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (provider_id) REFERENCES oauth_providers(id) ON DELETE CASCADE,
    UNIQUE KEY unique_provider_user (provider_id, provider_user_id),
    INDEX idx_user (user_id)
);

CREATE TABLE analytics_events (
    id BIGINTEGERPRIMARY KEY SERIAL,
    event_type VARCHAR(50) NOT NULL ,
    page_uuid VARCHAR(255),
    user_id INTEGER
    session_id VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    referer TEXT,
    metadata JSON ,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_event_type (event_type),
    INDEX idx_page (page_uuid),
    INDEX idx_created (created_at),
    INDEX idx_session (session_id)
);

CREATE TABLE backups (
    id INTEGERPRIMARY KEY SERIAL,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    type VARCHAR(50) NOT NULL ,
    status VARCHAR(50) DEFAULT 'pending' ,
    file_path VARCHAR(500),
    file_size BIGINTEGER,
    storage_location VARCHAR(100) ,
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    INDEX idx_status (status),
    INDEX idx_created (created_at)
);

-- Add tenant_id to existing tables (makes them multi-tenant aware)
-- Note: Run these ALTER statements carefully in production
-- ALTER TABLE pages ADD COLUMN tenant_id INTEGERAFTER id;
-- ALTER TABLE users ADD COLUMN tenant_id INTEGERAFTER id;
-- ALTER TABLE media ADD COLUMN tenant_id INTEGERAFTER id;
-- ALTER TABLE modules ADD COLUMN tenant_id INTEGERAFTER id;

-- Add foreign keys (after adding tenant_id columns)
-- ALTER TABLE pages ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE users ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE media ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE modules ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
