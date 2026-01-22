-- Fix order_id types to match BigInt PKs (orders.id is BIGINT)

ALTER TABLE crm_interactions ALTER COLUMN order_id TYPE BIGINT;
ALTER TABLE inventory_audit_log ALTER COLUMN order_id TYPE BIGINT;
