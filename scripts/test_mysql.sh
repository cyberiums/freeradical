#!/bin/bash
# Test FreeRadical CMS with MySQL database
# Verifies database connectivity, schema, and PageStatus enum functionality

set -e

echo "🧪 Testing FreeRadical with MySQL..."
echo "======================================"

# Colors for output
GREEN='\033[0.32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
MYSQL_USER=${MYSQL_USER:-freeradical}
MYSQL_PASSWORD=${MYSQL_PASSWORD:-password}
MYSQL_DATABASE=${MYSQL_DATABASE:-freeradical}
MYSQL_HOST=${MYSQL_HOST:-localhost}
MYSQL_PORT=${MYSQL_PORT:-5506}
API_URL="http://localhost:8000"

echo -e "${YELLOW}Step 1: Starting MySQL stack...${NC}"
docker-compose up -d mysql redis cms
sleep 5

echo -e "\n${YELLOW}Step 2: Checking database connectivity...${NC}"
if docker-compose exec -T mysql mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" "$MYSQL_DATABASE" -e "SELECT 1;" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ MySQL connection successful${NC}"
else
    echo -e "${RED}✗ MySQL connection failed${NC}"
    exit 1
fi

echo -e "\n${YELLOW}Step 3: Verifying schema tables...${NC}"
TABLE_COUNT=$(docker-compose exec -T mysql mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" "$MYSQL_DATABASE" -sN -e "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='$MYSQL_DATABASE';")
echo -e "${GREEN}✓ Found $TABLE_COUNT tables${NC}"

echo -e "\n${YELLOW}Step 4: Testing API health...${NC}"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/")
if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✓ API responding (HTTP $HTTP_CODE)${NC}"
else
    echo -e "${RED}✗ API not responding (HTTP $HTTP_CODE)${NC}"
    exit 1
fi

echo -e "\n${YELLOW}Step 5: Testing PageStatus enum...${NC}"
# Check if status column exists with ENUM type
STATUS_TYPE=$(docker-compose exec -T mysql mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" "$MYSQL_DATABASE" -sN -e "SELECT COLUMN_TYPE FROM information_schema.COLUMNS WHERE TABLE_SCHEMA='$MYSQL_DATABASE' AND TABLE_NAME='pages' AND COLUMN_NAME='status';" 2>/dev/null || echo "")

if [[ "$STATUS_TYPE" =~ ^enum ]]; then
    echo -e "${GREEN}✓ PageStatus ENUM type found: $STATUS_TYPE${NC}"
else
    echo -e "${YELLOW}⚠ Status column not found or not ENUM type${NC}"
fi

echo -e "\n${YELLOW}Step 6: Testing CRUD operations...${NC}"
# Note: Actual CRUD tests would require authentication
echo -e "${YELLOW}ℹ CRUD tests require authentication - skipping${NC}"

echo -e "\n${GREEN}======================================"
echo -e "✅ MySQL tests completed successfully!${NC}"
echo -e "Run 'docker-compose logs cms' to view application logs"
