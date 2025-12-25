-- Rollback semantic search

DROP TABLE IF EXISTS search_history;
DROP TABLE IF EXISTS content_embeddings;
DROP EXTENSION IF EXISTS vector;
