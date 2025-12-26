-- Add full-text search indexes using PostgreSQL text search

-- Pages full-text index (using GIN)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_pages_fulltext') THEN
        CREATE INDEX IF NOT EXISTS idx_pages_fulltext ON pages 
        USING GIN (to_tsvector('english', coalesce(page_title, '') || ' ' || coalesce(page_name, '') || ' ' || coalesce(meta_title, '') || ' ' || coalesce(meta_description, '')));
    END IF;
END $$;

-- Modules full-text index
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_modules_fulltext') THEN
        CREATE INDEX IF NOT EXISTS idx_modules_fulltext ON modules 
        USING GIN (to_tsvector('english', coalesce(title, '') || ' ' || coalesce(content, '')));
    END IF;
END $$;

-- Media full-text index  
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_media_fulltext') THEN
        CREATE INDEX IF NOT EXISTS idx_media_fulltext ON media 
        USING GIN (to_tsvector('english', coalesce(filename, '') || ' ' || coalesce(original_filename, '') || ' ' || coalesce(alt_text, '')));
    END IF;
END $$;
