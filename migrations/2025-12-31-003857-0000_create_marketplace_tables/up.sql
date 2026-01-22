CREATE TABLE IF NOT EXISTS marketplace_plugins (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    icon_url VARCHAR(500),
    status VARCHAR(50) DEFAULT 'active',
    developer_id INTEGER,
    price_cents INTEGER DEFAULT 0,
    downloads_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create tenant_plugins junction table for installed plugins per tenant
CREATE TABLE IF NOT EXISTS tenant_plugins (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id),
    plugin_id INTEGER NOT NULL REFERENCES marketplace_plugins(id),
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    settings JSONB,
    installed_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, plugin_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_marketplace_plugins_status ON marketplace_plugins(status);
CREATE INDEX IF NOT EXISTS idx_tenant_plugins_tenant ON tenant_plugins(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tenant_plugins_plugin ON tenant_plugins(plugin_id);
