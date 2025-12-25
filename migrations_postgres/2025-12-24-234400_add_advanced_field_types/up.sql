-- Add advanced field types to modules table

-- Add field_type enum column
ALTER TABLE modules
ADD COLUMN field_type ENUM(
  'text',
  'textarea', 
  'wysiwyg',
  'json',
  'number',
  'boolean',
  'date',
  'datetime',
  'file_reference',
  'page_reference',
  'select',
  'multi_select'
) DEFAULT 'text' AFTER content;

-- Add field configuration JSON (for field-specific settings)
ALTER TABLE modules
ADD COLUMN field_config TEXT

AFTER field_type;

-- Add field validation rules
ALTER TABLE modules
ADD COLUMN validation_rules TEXT

AFTER field_config;

-- Index for faster field type queries
CREATE INDEX idx_modules_field_type ON modules(field_type);
