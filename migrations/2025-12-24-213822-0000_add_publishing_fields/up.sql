-- Add scheduled publishing fields to pages
-- Iteration 4, Task 4

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='pages' AND column_name='status') THEN
        ALTER TABLE pages
        ADD COLUMN status VARCHAR(20) DEFAULT 'published' CHECK (status IN ('draft', 'scheduled', 'published', 'archived'));
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='pages' AND column_name='publish_at') THEN
        ALTER TABLE pages
        ADD COLUMN publish_at TIMESTAMP NULL;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='pages' AND column_name='unpublish_at') THEN
        ALTER TABLE pages
        ADD COLUMN unpublish_at TIMESTAMP NULL;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_status ON pages(status);
CREATE INDEX IF NOT EXISTS idx_publish_at ON pages(publish_at);
CREATE INDEX IF NOT EXISTS idx_scheduled ON pages(status, publish_at);
