-- Add enterprise features: multi-tenancy, OAuth, analytics

CREATE TABLE tenants (
    id INT PRIMARY KEY AUTO_INCREMENT,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    custom_domain VARCHAR(255),
    plan VARCHAR(50) DEFAULT 'free' COMMENT 'free, pro, enterprise',
    is_active BOOLEAN DEFAULT TRUE,
    settings JSON COMMENT 'Tenant-specific configuration',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_subdomain (subdomain),
    INDEX idx_custom_domain (custom_domain)
);

CREATE TABLE oauth_providers (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL COMMENT 'google, github, microsoft',
    client_id VARCHAR(255) NOT NULL,
    client_secret VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE user_oauth_connections (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    provider_id INT NOT NULL,
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
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    event_type VARCHAR(50) NOT NULL COMMENT 'page_view, click, conversion',
    page_uuid VARCHAR(255),
    user_id INT,
    session_id VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    referer TEXT,
    metadata JSON COMMENT 'Additional event data',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_event_type (event_type),
    INDEX idx_page (page_uuid),
    INDEX idx_created (created_at),
    INDEX idx_session (session_id)
);

CREATE TABLE backups (
    id INT PRIMARY KEY AUTO_INCREMENT,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    type VARCHAR(50) NOT NULL COMMENT 'full, incremental',
    status VARCHAR(50) DEFAULT 'pending' COMMENT 'pending, completed, failed',
    file_path VARCHAR(500),
    file_size BIGINT COMMENT 'Size in bytes',
    storage_location VARCHAR(100) COMMENT 's3, local, gcs',
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    INDEX idx_status (status),
    INDEX idx_created (created_at)
);

-- Add tenant_id to existing tables (makes them multi-tenant aware)
-- Note: Run these ALTER statements carefully in production
-- ALTER TABLE pages ADD COLUMN tenant_id INT AFTER id;
-- ALTER TABLE users ADD COLUMN tenant_id INT AFTER id;
-- ALTER TABLE media ADD COLUMN tenant_id INT AFTER id;
-- ALTER TABLE modules ADD COLUMN tenant_id INT AFTER id;

-- Add foreign keys (after adding tenant_id columns)
-- ALTER TABLE pages ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE users ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE media ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
-- ALTER TABLE modules ADD FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
