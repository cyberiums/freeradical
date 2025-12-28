CREATE TABLE tenant_sso_configs (
  id SERIAL PRIMARY KEY,
  tenant_id INTEGER NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  idp_entity_id VARCHAR(255) NOT NULL,
  idp_sso_url VARCHAR(500) NOT NULL,
  x509_certificate TEXT NOT NULL,
  is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT unique_tenant_sso UNIQUE (tenant_id)
);

CREATE INDEX idx_tenant_sso_configs_tenant_id ON tenant_sso_configs(tenant_id);
