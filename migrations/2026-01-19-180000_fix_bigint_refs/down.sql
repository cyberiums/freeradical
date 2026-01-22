-- Down migration
ALTER TABLE crm_interactions ALTER COLUMN order_id TYPE INTEGER;
ALTER TABLE inventory_audit_log ALTER COLUMN order_id TYPE INTEGER;
