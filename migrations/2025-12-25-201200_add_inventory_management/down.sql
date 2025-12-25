-- Remove inventory management fields
ALTER TABLE products
DROP COLUMN stock_quantity,
DROP COLUMN low_stock_threshold,
DROP COLUMN stock_status,
DROP COLUMN track_inventory,
DROP COLUMN allow_backorder,
DROP COLUMN backorder_limit;

-- Drop inventory audit log table
DROP TABLE IF EXISTS inventory_audit_log;

-- Drop product variants table
DROP TABLE IF EXISTS product_variants;
