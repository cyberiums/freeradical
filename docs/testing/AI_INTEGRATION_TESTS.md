# FreeRadical AI Features - Integration Test Plan

## Overview

Comprehensive testing plan for Phase 2 AI/MCP features to ensure production readiness.

---

## Test Coverage

### 1. AI Provider Management

**Tests:**
- ✅ Create provider with valid credentials
- ✅ Update provider configuration
- ✅ Delete provider
- ✅ List providers with pagination
- ✅ Test provider connection
- ✅ Budget limit enforcement
- ✅ Token usage tracking
- ⚠️ Admin-only access control

**Files to Test:**
- `src/services/ai_provider_service.rs`
- `src/models/ai_provider_models.rs`

**Test Commands:**
```bash
# Unit tests
cargo test ai_provider

# Integration tests
cargo test --test ai_provider_integration
```

---

### 2. AI Content Generation

**Tests:**
- ✅ Generate blog post
- ✅ Generate meta description
- ✅ Generate title
- ✅ Multi-provider fallback
- ✅ Cost calculation
- ✅ Usage logging
- ⚠️ Authentication required
- ⚠️ Rate limiting per user

**Files to Test:**
- `src/services/ai_content_service.rs`

**API Tests:**
```bash
# Test blog post generation
curl -X POST http://localhost:8001/ai/generate \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"prompt": "Rust programming", "content_type": "blog_post"}'

# Expected: 201 Created with generated content
```

---

### 3. Metadata Automation

**Tests:**
- ✅ Extract keywords
- ✅ Generate tags
- ✅ Suggest categories
- ✅ Generate alt text
- ✅ All metadata endpoint
- ⚠️ Access control

**Files to Test:**
- `src/services/metadata_automation_service.rs`

**API Tests:**
```bash
# Test keyword extraction
curl -X POST http://localhost:8001/ai/metadata/keywords \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"content": "Sample blog post...", "max_items": 10}'
```

---

### 4. Semantic Search

**Tests:**
- ✅ Create embedding
- ✅ Semantic search with similarity threshold
- ✅ Permission filtering (guest/user/admin)
- ✅ Vector index usage
- ✅ Deduplication by content hash
- ⚠️ User scoping

**Prerequisites:**
- PostgreSQL with pgvector extension
- OpenAI API key for embeddings

**Files to Test:**
- `src/services/semantic_search_service.rs`

**API Tests:**
```bash
# Create embedding
curl -X POST http://localhost:8001/search/embedding \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"page_id": 1, "content": "Test content"}'

# Semantic search
curl -X POST http://localhost:8001/search/semantic \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"query": "rust cms", "limit": 10, "min_similarity": 0.7}'
```

---

### 5. Recommendations

**Tests:**
- ✅ Get related content
- ✅ Get trending content
- ✅ Permission filtering
- ✅ Score ranking

**Files to Test:**
- `src/services/recommendation_service.rs`

**API Tests:**
```bash
# Get related content
curl -X POST http://localhost:8001/recommendations/related \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"page_id": 1, "limit": 5}'

# Get trending
curl http://localhost:8001/recommendations/trending?limit=10
```

---

### 6. Authorization & Scoping

**Tests:**
- ✅ Guest access (published only)
- ✅ User access (published + own)
- ✅ Admin access (all)
- ✅ JWT extraction
- ✅ Scope verification
- ✅ Page-level access control

**Files to Test:**
- `src/services/ai_authorization_service.rs`

**Test Scenarios:**
```bash
# Test guest (no auth) - should fail
curl -X POST http://localhost:8001/ai/generate \
  -d '{"prompt": "test"}'
# Expected: 401 Unauthorized

# Test user (with auth) - should succeed
curl -X POST http://localhost:8001/ai/generate \
  -H "Authorization: Bearer $USER_TOKEN" \
  -d '{"prompt": "test"}'
# Expected: 201 Created

# Test admin provider access
curl http://localhost:8001/admin/ai/providers \
  -H "Authorization: Bearer $ADMIN_TOKEN"
# Expected: 200 OK with provider list
```

---

## Performance Tests

### Load Testing

```bash
# Apache Bench - Content generation
ab -n 100 -c 10 -H "Authorization: Bearer $TOKEN" \
  -p payload.json \
  http://localhost:8001/ai/generate

# Target: <500ms p95 latency

# Semantic search
ab -n 1000 -c 50 \
  -p search.json \
  http://localhost:8001/search/semantic

# Target: <200ms p95 latency
```

### Database Performance

```sql
-- Check index usage
EXPLAIN ANALYZE
SELECT * FROM content_embeddings
ORDER BY embedding <=> '[...]'::vector
LIMIT 10;

-- Should use: Index Scan using idx_content_embeddings_vector
```

---

## End-to-End Tests

### User Journey 1: Content Creation Flow

1. User logs in
2. Generates blog post via AI
3. AI generates metadata (keywords, tags)
4. Content is saved with embeddings
5. Search finds the new content
6. Recommendations include the content

### User Journey 2: Search Discovery

1. Guest searches for topic
2. Gets results (published only)
3. Clicks on result
4. Sees related recommendations
5. All scoped to accessible content

---

## Security Tests

### Penetration Testing

```bash
# Try to access admin endpoint without admin role
curl -X POST http://localhost:8001/admin/ai/providers \
  -H "Authorization: Bearer $USER_TOKEN" \
  -d '{"provider_type": "openai"}'
# Expected: 403 Forbidden

# Try to search private content of other users
curl -X POST http://localhost:8001/search/semantic \
  -H "Authorization: Bearer $USER_TOKEN" \
  -d '{"query": "private content"}'
# Expected: Results filtered to accessible only

# Try SQL injection in search
curl -X POST http://localhost:8001/search/semantic \
  -d '{"query": "test\"; DROP TABLE pages; --"}'
# Expected: Safe, parameterized query
```

---

## Monitoring & Observability

### Metrics to Track

```rust
// Token usage per user
let tokens_used = track_usage(user_id, operation);

// API latency
let latency = measure_request_time();

// Error rate
let error_rate = errors / total_requests;

// Cache hit rate
let cache_hits = cached / total;
```

### Logs to Verify

```bash
# Check AI usage logs
SELECT user_id, operation, total_tokens, cost_cents
FROM ai_usage_log
WHERE created_at > NOW() - INTERVAL '1 day'
ORDER BY created_at DESC;

# Check search history
SELECT user_id, query, results_count
FROM search_history
WHERE created_at > NOW() - INTERVAL '1 hour';
```

---

## Test Environment Setup

### 1. Local Testing

```bash
# Start PostgreSQL with pgvector
docker run -d \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  ankane/pgvector

# Run migrations
diesel migration run

# Set environment variables
export OPENAI_API_KEY=sk-test...
export DATABASE_URL=postgresql://localhost/freeradical
```

### 2. CI/CD Testing

```yaml
# .github/workflows/test.yml
name: Test AI Features
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: ankane/pgvector
        env:
          POSTGRES_PASSWORD: password
    steps:
      - uses: actions/checkout@v2
      - run: cargo test
      - run: cargo test --test integration
```

---

## Success Criteria

### Phase 2 Complete When:

✅ All unit tests pass  
✅ All integration tests pass  
✅ API endpoints return <500ms (p95)  
✅ pgvector search <200ms (p95)  
✅ Authorization tests pass  
✅ No SQL injection vulnerabilities  
✅ User scoping verified  
✅ Documentation complete  
✅ Production deployment successful

---

## Test Execution Plan

### Week 11: Unit & Integration Tests

**Day 1-2:** AI provider tests  
**Day 3-4:** Content generation tests  
**Day 5-6:** Metadata & search tests  
**Day 7:** Recommendations & auth tests

### Week 12: Performance & Security

**Day 1-2:** Load testing  
**Day 3-4:** Security testing  
**Day 5:** E2E user journeys  
**Day 6-7:** Bug fixes & optimization

---

## Current Status

**Implemented:** 36 commits, 5,800+ lines  
**Tested:** Authorization service (unit tests)  
**Remaining:** Integration tests, performance tests, security tests

**Next:** Create integration test files and run test suite.
