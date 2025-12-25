-- Rollback advanced field types

DROP INDEX idx_modules_field_type ON modules;

ALTER TABLE modules
DROP COLUMN validation_rules,
DROP COLUMN field_config,
DROP COLUMN field_type;
