-- Convert tenants.settings to JSONB
ALTER TABLE tenants 
ALTER COLUMN settings TYPE JSONB 
USING settings::JSONB;

-- Convert audit_logs.details to JSONB
ALTER TABLE audit_logs 
ALTER COLUMN details TYPE JSONB 
USING details::JSONB;

-- Convert other JSON columns if any (webhook_logs.payload is usually JSONB but let's check schema.rs later if needed)
-- For now, fixing tenants.settings is the priority.
