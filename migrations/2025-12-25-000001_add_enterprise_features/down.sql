-- Rollback enterprise features

-- Remove foreign keys first
-- ALTER TABLE modules DROP FOREIGN KEY modules_ibfk_tenant;
-- ALTER TABLE media DROP FOREIGN KEY media_ibfk_tenant;
-- ALTER TABLE users DROP FOREIGN KEY users_ibfk_tenant;
-- ALTER TABLE pages DROP FOREIGN KEY pages_ibfk_tenant;

-- Remove tenant_id columns
-- ALTER TABLE modules DROP COLUMN tenant_id;
-- ALTER TABLE media DROP COLUMN tenant_id;
-- ALTER TABLE users DROP COLUMN tenant_id;
-- ALTER TABLE pages DROP COLUMN tenant_id;

DROP TABLE IF EXISTS backups;
DROP TABLE IF EXISTS analytics_events;
DROP TABLE IF EXISTS user_oauth_connections;
DROP TABLE IF EXISTS oauth_providers;
DROP TABLE IF EXISTS tenants;
