CREATE TABLE tenant_plugins (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plugin_id INTEGER NOT NULL REFERENCES marketplace_plugins(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    settings JSONB,
    installed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, plugin_id)
);

CREATE INDEX idx_tenant_plugins_tenant ON tenant_plugins(tenant_id);
