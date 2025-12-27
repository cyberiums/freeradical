-- Revert multi-tenancy changes
ALTER TABLE pages DROP COLUMN tenant_id;
ALTER TABLE products DROP COLUMN tenant_id;
ALTER TABLE orders DROP COLUMN tenant_id;
ALTER TABLE media DROP COLUMN tenant_id;

DROP TABLE tenant_members;
