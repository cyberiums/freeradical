-- Add advanced field types to modules table

-- Add field_type column (using VARCHAR instead of ENUM for flexibility/schema compatibility)
ALTER TABLE modules
ADD COLUMN field_type VARCHAR(30) DEFAULT 'text';

-- Add field configuration JSON (for field-specific settings)
ALTER TABLE modules
ADD COLUMN field_config TEXT;

-- Add field validation rules
ALTER TABLE modules
ADD COLUMN validation_rules TEXT;

-- Index for faster field type queries
CREATE INDEX idx_modules_field_type ON modules(field_type);
