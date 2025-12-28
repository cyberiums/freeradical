CREATE TABLE tenant_webhooks (
    id UUID PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id),
    url VARCHAR(2048) NOT NULL,
    secret VARCHAR(255) NOT NULL,
    events JSONB NOT NULL, -- List of event strings ["order.created", "product.updated"]
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tenant_webhooks_tenant ON tenant_webhooks(tenant_id);
