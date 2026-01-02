-- Revert content_embeddings alterations
DROP INDEX IF EXISTS idx_content_embeddings_dedup;
DROP INDEX IF EXISTS idx_content_embeddings_page_uuid;

ALTER TABLE content_embeddings 
    DROP COLUMN IF EXISTS content_preview,
    DROP COLUMN IF EXISTS embedding,
    DROP COLUMN IF EXISTS content_hash,
    DROP COLUMN IF EXISTS page_uuid;
