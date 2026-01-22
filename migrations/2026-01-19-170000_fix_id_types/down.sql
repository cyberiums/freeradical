-- Down migration
ALTER TABLE inventory_audit_log ALTER COLUMN product_id TYPE INTEGER;
ALTER TABLE product_variants ALTER COLUMN product_id TYPE INTEGER;
