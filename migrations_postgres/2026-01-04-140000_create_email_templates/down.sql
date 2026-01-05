-- Drop email_templates table
DROP INDEX IF EXISTS idx_email_templates_tenant_key;
DROP INDEX IF EXISTS idx_email_templates_active;
DROP TABLE IF EXISTS email_templates;
