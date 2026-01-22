-- Fix ID types to match BigInt PKs

-- products.id is BIGINT, so references must be BIGINT
ALTER TABLE inventory_audit_log ALTER COLUMN product_id TYPE BIGINT;
ALTER TABLE product_variants ALTER COLUMN product_id TYPE BIGINT;

-- Check if order_items exists and uses product_id
-- ALTER TABLE order_items ALTER COLUMN product_id TYPE BIGINT;
