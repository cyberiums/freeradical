-- Revert tenants.settings to JSON
ALTER TABLE tenants 
ALTER COLUMN settings TYPE JSON 
USING settings::JSON;

-- Revert audit_logs.details to JSON
ALTER TABLE audit_logs 
ALTER COLUMN details TYPE JSON 
USING details::JSON;
