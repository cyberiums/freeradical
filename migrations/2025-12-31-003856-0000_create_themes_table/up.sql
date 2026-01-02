-- Create themes table for theme management with multi-tenant support
CREATE TABLE themes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    thumbnail_url VARCHAR(500),
    is_active BOOLEAN DEFAULT true,
    is_default BOOLEAN DEFAULT false,
    tenant_id INTEGER REFERENCES tenants(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_themes_tenant ON themes(tenant_id);
CREATE INDEX idx_themes_active ON themes(is_active) WHERE is_active = true;
