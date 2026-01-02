-- Alter content_embeddings table to add missing columns required by semantic_search_service
ALTER TABLE content_embeddings 
    ADD COLUMN IF NOT EXISTS page_uuid VARCHAR(255),
    ADD COLUMN IF NOT EXISTS content_hash VARCHAR(64) NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS embedding TEXT,
    ADD COLUMN IF NOT EXISTS content_preview TEXT;

-- Create indexes for performance and deduplication
CREATE INDEX IF NOT EXISTS idx_content_embeddings_page_uuid ON content_embeddings(page_uuid);
CREATE UNIQUE INDEX IF NOT EXISTS idx_content_embeddings_dedup ON content_embeddings(page_uuid, content_hash);
