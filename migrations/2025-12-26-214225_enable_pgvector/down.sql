-- Revert pgvector changes
DROP INDEX IF EXISTS idx_content_embeddings_content_type;
DROP INDEX IF EXISTS idx_content_embeddings_page_id;
DROP INDEX IF EXISTS content_embeddings_vector_idx;

-- Revert vector column back to TEXT
ALTER TABLE content_embeddings 
  ALTER COLUMN embedding_vector TYPE TEXT 
  USING embedding_vector::text;

-- Drop the extension
DROP EXTENSION IF EXISTS vector;
