-- Add advanced field types to modules table

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='modules' AND column_name='field_type') THEN
        ALTER TABLE modules
        ADD COLUMN field_type VARCHAR(30) DEFAULT 'text' 
        CHECK (field_type IN ('text', 'textarea', 'wysiwyg', 'json', 'number', 'boolean', 'date', 'datetime', 'file_reference', 'page_reference', 'select', 'multi_select'));
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='modules' AND column_name='field_config') THEN
        ALTER TABLE modules
        ADD COLUMN field_config TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='modules' AND column_name='validation_rules') THEN
        ALTER TABLE modules
        ADD COLUMN validation_rules TEXT;
    END IF;
END $$;

-- Index for faster field type queries
CREATE INDEX IF NOT EXISTS idx_modules_field_type ON modules(field_type);
