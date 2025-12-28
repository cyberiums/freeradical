ALTER TABLE crm_customers ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE crm_interactions ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE crm_segments ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE crm_campaigns ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE crm_tasks ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE crm_notes ADD COLUMN tenant_id INT REFERENCES tenants(id) ON DELETE CASCADE;

CREATE INDEX idx_crm_customers_tenant ON crm_customers(tenant_id);
CREATE INDEX idx_crm_interactions_tenant ON crm_interactions(tenant_id);
CREATE INDEX idx_crm_segments_tenant ON crm_segments(tenant_id);
CREATE INDEX idx_crm_campaigns_tenant ON crm_campaigns(tenant_id);
CREATE INDEX idx_crm_tasks_tenant ON crm_tasks(tenant_id);
CREATE INDEX idx_crm_notes_tenant ON crm_notes(tenant_id);
