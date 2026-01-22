-- Content Embeddings for Semantic Search
DROP TABLE IF EXISTS content_embeddings CASCADE;
CREATE TABLE IF NOT EXISTS content_embeddings (
    id BIGSERIAL PRIMARY KEY,
    page_id INTEGER,
    embedding_vector FLOAT8[],
    model_name VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_embeddings_page ON content_embeddings(page_id);
