-- Make SAML fields nullable
ALTER TABLE tenant_sso_configs ALTER COLUMN idp_entity_id DROP NOT NULL;
ALTER TABLE tenant_sso_configs ALTER COLUMN idp_sso_url DROP NOT NULL;
ALTER TABLE tenant_sso_configs ALTER COLUMN x509_certificate DROP NOT NULL;

-- Add OIDC/Common fields
ALTER TABLE tenant_sso_configs ADD COLUMN provider_type VARCHAR(50) DEFAULT 'saml';
ALTER TABLE tenant_sso_configs ADD COLUMN client_id VARCHAR(255);
ALTER TABLE tenant_sso_configs ADD COLUMN client_secret VARCHAR(500); -- Encrypted
ALTER TABLE tenant_sso_configs ADD COLUMN discovery_url VARCHAR(500);
