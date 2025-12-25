#!/bin/bash
# Performance Benchmark Script: MySQL vs PostgreSQL
# Tests database query performance, API response times, and throughput

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

RESULTS_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).md"

echo -e "${BLUE}🔬 FreeRadical CMS - Database Performance Benchmark${NC}"
echo -e "${BLUE}=================================================${NC}\n"

# Initialize results file
cat > "$RESULTS_FILE" << 'EOF'
# Database Performance Benchmark Results

**Test Date:** $(date)
**Test Duration:** Calculating...

## Test Configuration

- **Application:** FreeRadical CMS v1.0.3
- **Environment:** Docker (localhost)
- **Test Tool:** curl + time
- **Metrics:** Response time, throughput, connection latency

EOF

benchmark_database() {
    local db_name=$1
    local api_url=$2
    local compose_file=$3
    
    echo -e "\n${YELLOW}===============================================${NC}"
    echo -e "${YELLOW}Testing: $db_name${NC}"
    echo -e "${YELLOW}===============================================${NC}\n"
    
    # Start the database stack
    echo -e "${BLUE}1. Starting $db_name stack...${NC}"
    if [ "$compose_file" = "docker-compose.yml" ]; then
        docker-compose up -d > /dev/null 2>&1
    else
        docker-compose -f "$compose_file" up -d > /dev/null 2>&1
    fi
    
    sleep 8
    
    # Test 1: API Health Check
    echo -e "${BLUE}2. API Health Check${NC}"
    start_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
    http_code=$(curl -s -o /dev/null -w "%{http_code}" "$api_url/" || echo "000")
    end_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
    health_time=$((end_time - start_time))
    
    if [ "$http_code" = "200" ]; then
        echo -e "${GREEN}   ✓ API responding (${health_time}ms)${NC}"
    else
        echo -e "${RED}   ✗ API not responding (HTTP $http_code)${NC}"
        return 1
    fi
    
    # Test 2: Page List Endpoint (Read Performance)
    echo -e "${BLUE}3. Testing Read Performance (GET /api/pages)${NC}"
    total_time=0
    success_count=0
    iterations=10
    
    for i in $(seq 1 $iterations); do
        start_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
        response=$(curl -s "$api_url/api/pages" 2>/dev/null)
        end_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
        request_time=$((end_time - start_time))
        
        if [ -n "$response" ]; then
            success_count=$((success_count + 1))
            total_time=$((total_time + request_time))
        fi
    done
    
    avg_read_time=$((total_time / iterations))
    echo -e "${GREEN}   ✓ Average: ${avg_read_time}ms (${success_count}/${iterations} successful)${NC}"
    
    # Test 3: Homepage Load (Template Rendering)
    echo -e "${BLUE}4. Testing Homepage Rendering${NC}"
    total_time=0
    iterations=5
    
    for i in $(seq 1 $iterations); do
        start_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
        curl -s "$api_url/" > /dev/null 2>&1
        end_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
        request_time=$((end_time - start_time))
        total_time=$((total_time + request_time))
    done
    
    avg_render_time=$((total_time / iterations))
    echo -e "${GREEN}   ✓ Average: ${avg_render_time}ms${NC}"
    
    # Test 4: Concurrent Requests (Throughput)
    echo -e "${BLUE}5. Testing Concurrent Request Handling${NC}"
    start_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
    
    for i in $(seq 1 20); do
        curl -s "$api_url/" > /dev/null 2>&1 &
    done
    wait
    
    end_time=$(python3 -c 'import time; print(int(time.time() * 1000))')
    concurrent_time=$((end_time - start_time))
    throughput=$(echo "scale=2; 20000 / $concurrent_time" | bc)
    
    echo -e "${GREEN}   ✓ 20 requests in ${concurrent_time}ms (~${throughput} req/sec)${NC}"
    
    # Save results
    cat >> "$RESULTS_FILE" << EOF

## $db_name Results

| Metric | Value |
|--------|-------|
| API Health Check | ${health_time}ms |
| Avg Read (GET /api/pages) | ${avg_read_time}ms |
| Avg Homepage Render | ${avg_render_time}ms |
| 20 Concurrent Requests | ${concurrent_time}ms (~${throughput} req/sec) |

EOF
    
    # Store for comparison
    if [ "$db_name" = "MySQL" ]; then
        mysql_read=$avg_read_time
        mysql_render=$avg_render_time
        mysql_concurrent=$concurrent_time
    else
        postgres_read=$avg_read_time
        postgres_render=$avg_render_time
        postgres_concurrent=$concurrent_time
    fi
    
    # Cleanup
    echo -e "${BLUE}6. Stopping $db_name stack...${NC}"
    if [ "$compose_file" = "docker-compose.yml" ]; then
        docker-compose down > /dev/null 2>&1
    else
        docker-compose -f "$compose_file" down > /dev/null 2>&1
    fi
    
    sleep 3
}

# Run benchmarks
benchmark_database "MySQL" "http://localhost:8000" "docker-compose.yml"
benchmark_database "PostgreSQL" "http://localhost:8001" "docker-compose.postgres.yml"

# Generate comparison
echo -e "\n${BLUE}=================================================${NC}"
echo -e "${BLUE}Benchmark Summary${NC}"
echo -e "${BLUE}=================================================${NC}\n"

# Calculate differences
read_diff=$((postgres_read - mysql_read))
render_diff=$((postgres_render - mysql_render))
concurrent_diff=$((postgres_concurrent - mysql_concurrent))

cat >> "$RESULTS_FILE" << EOF

## Performance Comparison

| Metric | MySQL | PostgreSQL | Difference | Winner |
|--------|-------|------------|------------|--------|
| Read Performance | ${mysql_read}ms | ${postgres_read}ms | ${read_diff}ms | $([ $mysql_read -lt $postgres_read ] && echo "MySQL" || echo "PostgreSQL") |
| Rendering | ${mysql_render}ms | ${postgres_render}ms | ${render_diff}ms | $([ $mysql_render -lt $postgres_render ] && echo "MySQL" || echo "PostgreSQL") |
| Concurrent (20 req) | ${mysql_concurrent}ms | ${postgres_concurrent}ms | ${concurrent_diff}ms | $([ $mysql_concurrent -lt $postgres_concurrent ] && echo "MySQL" || echo "PostgreSQL") |

## Recommendations

EOF

# Add recommendations based on results
if [ $mysql_read -lt $postgres_read ]; then
    echo "- **Read-heavy workloads**: MySQL shows better read performance" >> "$RESULTS_FILE"
else
    echo "- **Read-heavy workloads**: PostgreSQL shows better read performance" >> "$RESULTS_FILE"
fi

if [ $mysql_concurrent -lt $postgres_concurrent ]; then
    echo "- **High concurrency**: MySQL handles concurrent requests better" >> "$RESULTS_FILE"
else
    echo "- **High concurrency**: PostgreSQL handles concurrent requests better" >> "$RESULTS_FILE"
fi

cat >> "$RESULTS_FILE" << 'EOF'

## Notes

- Tests performed on localhost Docker environment
- Results may vary in production with different hardware/network
- Both databases use default Docker configurations
- No query optimization or indexing tuning applied
- Cold start times not included in averages

EOF

echo -e "${GREEN}✅ Benchmark Complete!${NC}"
echo -e "${GREEN}Results saved to: $RESULTS_FILE${NC}\n"

# Display summary
echo -e "${YELLOW}Quick Summary:${NC}"
echo -e "  MySQL Read:        ${mysql_read}ms"
echo -e "  PostgreSQL Read:   ${postgres_read}ms"
echo ""
echo -e "  MySQL Render:      ${mysql_render}ms"
echo -e "  PostgreSQL Render: ${postgres_render}ms"
echo ""
echo -e "  MySQL Concurrent:      ${mysql_concurrent}ms"
echo -e "  PostgreSQL Concurrent: ${postgres_concurrent}ms"

cat "$RESULTS_FILE"
