-- Semantic Search with pgvector
-- Enable vector extension for content embeddings

-- Install pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- Content embeddings table for semantic search
CREATE TABLE content_embeddings (
    id BIGSERIAL PRIMARY KEY,
    page_id BIGINT REFERENCES pages(id),
    content_hash VARCHAR(64) NOT NULL,
    embedding vector(1536), -- OpenAI text-embedding-ada-002 dimension
    content_preview TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(page_id, content_hash)
);

-- Create indexes for vector search
CREATE INDEX idx_content_embeddings_page ON content_embeddings(page_id);
CREATE INDEX idx_content_embeddings_hash ON content_embeddings(content_hash);

-- IVFFlat index for fast similarity search (cosine distance)
CREATE INDEX idx_content_embeddings_vector ON content_embeddings 
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- Search history for analytics
CREATE TABLE search_history (
    id BIGSERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    query TEXT NOT NULL,
    search_type VARCHAR(20) DEFAULT 'semantic', -- 'semantic', 'keyword', 'hybrid'
    results_count INTEGER,
    top_result_id BIGINT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_search_history_user ON search_history(user_id);
CREATE INDEX idx_search_history_created ON search_history(created_at);
