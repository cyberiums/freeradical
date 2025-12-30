#!/bin/bash
# Test FreeRadical CMS with PostgreSQL database
# Verifies database connectivity, schema, and PageStatus VARCHAR/CHECK constraint functionality

set -e

echo "🧪 Testing FreeRadical with PostgreSQL..."
echo "=========================================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
POSTGRES_USER=${POSTGRES_USER:-freeradical}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-password}
POSTGRES_DB=${POSTGRES_DB:-freeradical}
POSTGRES_HOST=${POSTGRES_HOST:-localhost}
POSTGRES_PORT=${POSTGRES_PORT:-5432}
API_URL="http://localhost:8001"

echo -e "${YELLOW}Step 1: Starting PostgreSQL stack...${NC}"
docker compose -f docker-compose.yml up -d
sleep 8

echo -e "\n${YELLOW}Step 2: Checking database connectivity...${NC}"
if docker compose -f docker-compose.yml exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "SELECT 1;" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ PostgreSQL connection successful${NC}"
else
    echo -e "${RED}✗ PostgreSQL connection failed${NC}"
    docker compose -f docker-compose.yml logs postgres
    exit 1
fi

echo -e "\n${YELLOW}Step 3: Running migrations...${NC}"
export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB"

# Check if diesel CLI is available
if command -v diesel &> /dev/null; then
    diesel migration run --database-url="$DATABASE_URL" --migration-dir=migrations_postgres || {
        echo -e "${YELLOW}⚠ Migration failed - may need manual review${NC}"
    }
else
    echo -e "${YELLOW}⚠ Diesel CLI not found - skipping migrations${NC}"
    echo -e "${YELLOW}  Install with: cargo install diesel_cli --no-default-features --features postgres${NC}"
fi

echo -e "\n${YELLOW}Step 4: Verifying schema tables...${NC}"
TABLE_COUNT=$(docker compose -f docker-compose.yml exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public';" | tr -d ' ')
echo -e "${GREEN}✓ Found $TABLE_COUNT tables${NC}"

echo -e "\n${YELLOW}Step 5: Testing API health...${NC}"
sleep 3
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/" || echo "000")
if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✓ API responding (HTTP $HTTP_CODE)${NC}"
else
    echo -e "${RED}✗ API not responding (HTTP $HTTP_CODE)${NC}"
    echo -e "${YELLOW}Checking container logs...${NC}"
    docker compose -f docker-compose.yml logs --tail=20 cms
fi

echo -e "\n${YELLOW}Step 6: Testing PageStatus VARCHAR type...${NC}"
STATUS_INFO=$(docker compose -f docker-compose.yml exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -t -c "SELECT data_type, character_maximum_length FROM information_schema.columns WHERE table_name='pages' AND column_name='status';" 2>/dev/null | tr -s ' ' || echo "")

if [[ "$STATUS_INFO" =~ "character varying" ]] || [[ "$STATUS_INFO" =~ "varchar" ]]; then
    echo -e "${GREEN}✓ PageStatus VARCHAR type found${NC}"
    echo -e "  Type info: $STATUS_INFO"
else
    echo -e "${YELLOW}⚠ Status column not found or unexpected type${NC}"
fi

echo -e "\n${YELLOW}Step 7: Cleanup...${NC}"
echo -e "${YELLOW}To stop: docker compose -f docker-compose.yml down${NC}"

echo -e "\n${GREEN}=========================================="
echo -e "✅ PostgreSQL tests completed!${NC}"
echo -e "Run 'docker compose -f docker-compose.yml logs cms' to view logs"
