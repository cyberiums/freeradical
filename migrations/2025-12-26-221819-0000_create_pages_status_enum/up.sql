-- Create enum type for page status
CREATE TYPE pages_status AS ENUM ('draft', 'published', 'archived', 'scheduled');

-- Add status column to pages table with enum type
DO $$ 
BEGIN
    -- Check if status column exists as VARCHAR
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'pages' AND column_name = 'status' AND data_type = 'character varying'
    ) THEN
        -- Remove default first to avoid cast issues
        ALTER TABLE pages ALTER COLUMN status DROP DEFAULT;
        -- Convert existing VARCHAR to ENUM
        ALTER TABLE pages ALTER COLUMN status TYPE pages_status USING status::pages_status;
        -- Re-add default
        ALTER TABLE pages ALTER COLUMN status SET DEFAULT 'draft'::pages_status;
    ELSIF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'pages' AND column_name = 'status'
    ) THEN
        -- Add column if it doesn't exist
        ALTER TABLE pages ADD COLUMN status pages_status DEFAULT 'draft';
    END IF;
END $$;
