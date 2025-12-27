-- Create tenant_members table (Many-to-Many User <-> Tenant)
CREATE TABLE tenant_members (
  id SERIAL PRIMARY KEY,
  tenant_id INT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role VARCHAR(50) NOT NULL DEFAULT 'viewer',
  status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, invited, suspended
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(tenant_id, user_id)
);

-- Add tenant_id to content tables
ALTER TABLE pages ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE products ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE orders ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE media ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;

-- Add indexes for performance
CREATE INDEX idx_tenant_members_user ON tenant_members(user_id);
CREATE INDEX idx_tenant_members_tenant ON tenant_members(tenant_id);
CREATE INDEX idx_pages_tenant ON pages(tenant_id);
CREATE INDEX idx_products_tenant ON products(tenant_id);
CREATE INDEX idx_orders_tenant ON orders(tenant_id);
CREATE INDEX idx_media_tenant ON media(tenant_id);
