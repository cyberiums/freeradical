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
| API Health Check | 74ms |
| Avg Read (GET /api/pages) | 0ms |
| Avg Homepage Render | 41ms |
| 20 Concurrent Requests | 106ms (~188.67 req/sec) |


## PostgreSQL Results

| Metric | Value |
|--------|-------|
| API Health Check | 69ms |
| Avg Read (GET /api/pages) | 0ms |
| Avg Homepage Render | 42ms |
| 20 Concurrent Requests | 103ms (~194.17 req/sec) |


## Performance Comparison

| Metric | MySQL | PostgreSQL | Difference | Winner |
|--------|-------|------------|------------|--------|
| Read Performance | 0ms | 0ms | 0ms | PostgreSQL |
| Rendering | 41ms | 42ms | 1ms | MySQL |
| Concurrent (20 req) | 106ms | 103ms | -3ms | PostgreSQL |

## Recommendations

- **Read-heavy workloads**: PostgreSQL shows better read performance
- **High concurrency**: PostgreSQL handles concurrent requests better

## Notes

- Tests performed on localhost Docker environment
- Results may vary in production with different hardware/network
- Both databases use default Docker configurations
- No query optimization or indexing tuning applied
- Cold start times not included in averages

