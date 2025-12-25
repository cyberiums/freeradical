# Database Performance Benchmark Results

**Test Date:** $(date)
**Test Duration:** Calculating...

## Test Configuration

- **Application:** FreeRadical CMS v1.0.3
- **Environment:** Docker (localhost)
- **Test Tool:** curl + time
- **Metrics:** Response time, throughput, connection latency


## MySQL Results

| Metric | Value |
|--------|-------|
| API Health Check | 59ms |
| Avg Read (GET /api/pages) | 0ms |
| Avg Homepage Render | 53ms |
| 20 Concurrent Requests | 117ms (~170.94 req/sec) |

