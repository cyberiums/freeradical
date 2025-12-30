ALTER TABLE tenant_sso_configs DROP COLUMN discovery_url;
ALTER TABLE tenant_sso_configs DROP COLUMN client_secret;
ALTER TABLE tenant_sso_configs DROP COLUMN client_id;
ALTER TABLE tenant_sso_configs DROP COLUMN provider_type;

-- Note: Cannot easily restore NOT NULL constraint without data cleanup
