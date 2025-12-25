# Using pgvector with Google Cloud SQL

## Overview

FreeRadical's semantic search is compatible with Google Cloud SQL for PostgreSQL with the pgvector extension.

**Reference:** [Google Cloud Blog - pgvector with LLMs](https://cloud.google.com/blog/products/databases/using-pgvector-llms-and-langchain-with-google-cloud-databases)

---

## Setup for Google Cloud SQL

### 1. Enable pgvector Extension

```sql
-- In Cloud SQL PostgreSQL instance
CREATE EXTENSION IF NOT EXISTS vector;
```

**Requirements:**
- PostgreSQL 12+ (Cloud SQL)
- pgvector extension enabled on instance

### 2. Cloud SQL Configuration

```bash
# Enable pgvector in Cloud SQL instance
gcloud sql instances patch INSTANCE_NAME \
  --database-flags=cloudsql.enable_pgvector=on
```

### 3. Connection String

```env
# .env
DATABASE_URL=postgresql://user:password@/dbname?host=/cloudsql/PROJECT:REGION:INSTANCE
```

---

## Compatibility Notes

### Vector Dimensions

FreeRadical uses **1536 dimensions** (OpenAI text-embedding-ada-002):

```sql
CREATE TABLE content_embeddings (
    embedding vector(1536)  -- Compatible with Cloud SQL
);
```

### Index Type

Cloud SQL supports IVFFlat indexing:

```sql
CREATE INDEX idx_content_embeddings_vector 
ON content_embeddings 
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);
```

**Tuning for Cloud SQL:**
- `lists = 100` for <1M vectors
- `lists = 1000` for 1M+ vectors

### Distance Operators

All pgvector operators work in Cloud SQL:

```sql
-- Cosine distance (used by FreeRadical)
embedding <=> query_vector

-- L2 distance
embedding <-> query_vector

-- Inner product
embedding <#> query_vector
```

---

## Performance Optimization

### 1. Connection Pooling

```rust
// Recommended for Cloud SQL
let pool = Pool::builder()
    .max_size(15)  // Cloud SQL default max connections
    .connection_timeout(Duration::from_secs(30))
    .build(manager)?;
```

### 2. Query Optimization

```sql
-- Use prepared statements
PREPARE semantic_search AS
SELECT page_id, 
       content_preview, 
       1 - (embedding <=> $1) AS similarity
FROM content_embeddings
WHERE 1 - (embedding <=> $1) > $2
ORDER BY embedding <=> $1
LIMIT $3;

-- Execute
EXECUTE semantic_search('[0.1, 0.2, ...]', 0.7, 10);
```

### 3. Index Maintenance

```sql
-- Rebuild index periodically
REINDEX INDEX idx_content_embeddings_vector;
```

---

## Cloud SQL Specific Features

### High Availability

```bash
# Enable HA for production
gcloud sql instances patch INSTANCE_NAME \
  --availability-type=REGIONAL
```

### Backups

```bash
# Automated backups (includes vector data)
gcloud sql instances patch INSTANCE_NAME \
  --backup-start-time=03:00
```

### Read Replicas

Vector search can use read replicas:

```env
# Primary for writes
DATABASE_URL=postgresql://...

# Replica for reads (search)
DATABASE_READ_URL=postgresql://replica-host/...
```

---

## Migration from Local PostgreSQL

### 1. Export Data

```bash
# Export embeddings
pg_dump -t content_embeddings local_db > embeddings.sql
```

### 2. Import to Cloud SQL

```bash
# Import to Cloud SQL
gcloud sql import sql INSTANCE_NAME gs://BUCKET/embeddings.sql
```

### 3. Rebuild Indexes

```sql
-- After import
REINDEX INDEX idx_content_embeddings_vector;
ANALYZE content_embeddings;
```

---

## Cost Optimization

### Storage

Vector embeddings use ~6KB per page (1536 dims × 4 bytes):

```
1,000 pages   = ~6 MB
10,000 pages  = ~60 MB
100,000 pages = ~600 MB
```

### Compute

- **Shared-core:** Development/testing
- **Standard (2 vCPU, 7.5GB):** Production <10K vectors
- **High-mem (4 vCPU, 26GB):** Production >10K vectors

---

## Monitoring

### Query Performance

```sql
-- Check index usage
SELECT schemaname, tablename, indexname, idx_scan
FROM pg_stat_user_indexes
WHERE tablename = 'content_embeddings';
```

### Storage

```sql
-- Check table size
SELECT pg_size_pretty(pg_total_relation_size('content_embeddings'));
```

---

## Troubleshooting

### Extension Not Available

```bash
# Check pgvector is enabled
gcloud sql instances describe INSTANCE_NAME \
  --format="value(settings.databaseFlags)"
```

### Slow Queries

```sql
-- Check if index is being used
EXPLAIN ANALYZE
SELECT * FROM content_embeddings
ORDER BY embedding <=> '[...]'
LIMIT 10;
```

Should show `Index Scan using idx_content_embeddings_vector`.

### Connection Limits

```bash
# Increase max connections
gcloud sql instances patch INSTANCE_NAME \
  --database-flags=max_connections=100
```

---

## Production Checklist

✅ pgvector extension enabled  
✅ IVFFlat index created  
✅ Connection pooling configured  
✅ High availability enabled  
✅ Automated backups scheduled  
✅ Monitoring alerts set up  
✅ Index maintenance scheduled

---

## Example: Full Setup

```bash
# 1. Create Cloud SQL instance
gcloud sql instances create freeradical-db \
  --database-version=POSTGRES_15 \
  --tier=db-custom-2-7680 \
  --region=us-central1 \
  --database-flags=cloudsql.enable_pgvector=on

# 2. Create database
gcloud sql databases create freeradical \
  --instance=freeradical-db

# 3. Enable pgvector
gcloud sql connect freeradical-db --database=freeradical
> CREATE EXTENSION vector;

# 4. Run migrations
diesel migration run --database-url=$DATABASE_URL

# 5. Verify
gcloud sql connect freeradical-db --database=freeradical
> SELECT * FROM pg_extension WHERE extname = 'vector';
```

---

## Resources

- [Google Cloud SQL PostgreSQL](https://cloud.google.com/sql/docs/postgres)
- [pgvector Documentation](https://github.com/pgvector/pgvector)
- [FreeRadical Semantic Search](./SEMANTIC_SEARCH.md)

---

**Status:** FreeRadical semantic search is fully compatible with Google Cloud SQL! ✅
