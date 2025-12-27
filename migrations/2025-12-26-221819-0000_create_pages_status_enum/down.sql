-- Revert status column back to VARCHAR if it was changed
DO $$ 
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'pages' AND column_name = 'status' AND udt_name = 'pages_status'
    ) THEN
        ALTER TABLE pages ALTER COLUMN status TYPE VARCHAR(9) USING status::VARCHAR;
    END IF;
END $$;

-- Drop the enum type
DROP TYPE IF EXISTS pages_status;
