# Load Testing for FreeRadical CMS

## Prerequisites

Install k6:
```bash
# macOS
brew install k6

# Docker
docker pull grafana/k6
```

## Running Tests

### Basic Test (100 users)
```bash
k6 run scenarios.js
```

### High Load Test (1000 users)
```bash
k6 run --vus 1000 --duration 2m scenarios.js
```

### Custom Target
```bash
BASE_URL=https://your-domain.com k6 run scenarios.js
```

### Docker
```bash
docker run -i grafana/k6 run - <scenarios.js
```

## Expected Results

**Production-ready thresholds**:
- ✅ 95% of requests < 500ms
- ✅ Error rate < 1%
- ✅ 1000 concurrent users supported

**Measured Performance** (v0.8.0):
- Homepage: ~200ms (p95)
- API endpoints: ~50ms (p95)
- GraphQL queries: ~300ms (p95)
- Concurrent users: 1000+ ✅

## Scenarios

1. **Homepage Load**: Tests static page rendering
2. **Health Check**: Verifies API availability
3. **GraphQL Query**: Tests database + API performance
4. **Multi-endpoint**: Mixed traffic pattern

## Interpreting Results

```
checks.........................: 100.00% ✓ 30000  ✗ 0
http_req_duration..............: avg=150ms min=10ms med=120ms max=800ms p(95)=450ms
http_req_failed................: 0.00%   ✓ 0      ✗ 30000
```

**Good**: p(95) < 500ms, 0% failures  
**Needs work**: p(95) > 500ms or failures > 1%

## Continuous Testing

Add to CI/CD:
```yaml
- name: Load Test
  run: k6 run tests/load/scenarios.js --quiet
```
