# FreeRadical CMS - Performance Benchmarking Script

## Usage
```bash
# Run all benchmarks
./benchmark.sh

# Run specific benchmark
./benchmark.sh homepage
./benchmark.sh dashboard
```

## Benchmarks

### Homepage Performance
- Target: >2,000 req/s
- Test with: wrk or ab

### Dashboard API
- Target: <100ms response
- All 4 endpoints

### Database Queries
- Target: <10ms
- Check with EXPLAIN

## Results

Results saved to: `benchmark_results.txt`
