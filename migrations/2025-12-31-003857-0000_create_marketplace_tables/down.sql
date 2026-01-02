-- Drop marketplace tables in reverse order (junction table first)
DROP TABLE IF EXISTS tenant_plugins CASCADE;
DROP TABLE IF EXISTS marketplace_plugins CASCADE;
