-- Enable pgvector extension for vector similarity search
CREATE EXTENSION IF NOT EXISTS vector;

-- Update the embedding_vector column type from TEXT to vector(1536)
-- Note: If column is already vector type, this will be a no-op
ALTER TABLE content_embeddings 
  ALTER COLUMN embedding_vector TYPE vector(1536) 
  USING CASE 
    WHEN embedding_vector IS NULL THEN NULL
    ELSE embedding_vector::vector
  END;

-- Create vector index for fast similarity search using IVFFlat algorithm
-- This uses cosine distance (1 - cosine similarity)
CREATE INDEX IF NOT EXISTS content_embeddings_vector_idx 
  ON content_embeddings 
  USING ivfflat (embedding_vector vector_cosine_ops)
  WITH (lists = 100);

-- Add index for efficient page lookups
CREATE INDEX IF NOT EXISTS idx_content_embeddings_page_id 
  ON content_embeddings(page_id) 
  WHERE page_id IS NOT NULL;
