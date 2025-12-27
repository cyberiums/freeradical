#!/bin/bash
set -e

echo "=================================================="
echo "pgvector Installation Script for Ubuntu/Docker"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PGVECTOR_VERSION="v0.5.1"
PGVECTOR_REPO="https://github.com/pgvector/pgvector.git"

echo -e "${YELLOW}Step 1: Checking system...${NC}"
if [ ! -f /etc/os-release ]; then
    echo -e "${RED}Error: Cannot detect OS${NC}"
    exit 1
fi

. /etc/os-release
if [[ "$ID" != "ubuntu" ]] && [[ "$ID" != "debian" ]]; then
    echo -e "${YELLOW}Warning: This script is designed for Ubuntu/Debian${NC}"
fi

echo -e "${GREEN}✓ Running on: $PRETTY_NAME${NC}"

echo -e "\n${YELLOW}Step 2: Detecting PostgreSQL version...${NC}"
PG_VERSION=$(psql --version 2>/dev/null | grep -oP '\d+' | head -1 || echo "")

if [ -z "$PG_VERSION" ]; then
    echo -e "${YELLOW}PostgreSQL client not found, trying to detect from server...${NC}"
    # Try to detect from installed packages
    PG_VERSION=$(dpkg -l | grep postgresql-server-dev | grep -oP 'postgresql-server-dev-\K\d+' | head -1 || echo "14")
fi

if [ -z "$PG_VERSION" ]; then
    PG_VERSION="14"
    echo -e "${YELLOW}Could not detect PostgreSQL version, defaulting to: $PG_VERSION${NC}"
else
    echo -e "${GREEN}✓ Detected PostgreSQL version: $PG_VERSION${NC}"
fi

echo -e "\n${YELLOW}Step 3: Installing build dependencies...${NC}"
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq

echo "Installing PostgreSQL development headers..."
apt-get install -y -qq \
    postgresql-server-dev-$PG_VERSION \
    build-essential \
    git \
    wget \
    curl

echo -e "${GREEN}✓ Dependencies installed${NC}"

echo -e "\n${YELLOW}Step 4: Downloading pgvector...${NC}"
WORK_DIR="/tmp/pgvector-install"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

git clone --depth 1 --branch "$PGVECTOR_VERSION" "$PGVECTOR_REPO"
cd pgvector

echo -e "${GREEN}✓ Downloaded pgvector $PGVECTOR_VERSION${NC}"

echo -e "\n${YELLOW}Step 5: Building pgvector...${NC}"
make clean || true
make

echo -e "${GREEN}✓ Build successful${NC}"

echo -e "\n${YELLOW}Step 6: Installing pgvector...${NC}"
make install

echo -e "${GREEN}✓ pgvector installed${NC}"

echo -e "\n${YELLOW}Step 7: Verifying installation...${NC}"

# Check if extension files exist
EXTENSION_DIR=$(pg_config --sharedir)/extension
if [ -f "$EXTENSION_DIR/vector.control" ]; then
    echo -e "${GREEN}✓ Extension control file found: $EXTENSION_DIR/vector.control${NC}"
else
    echo -e "${RED}✗ Extension control file not found${NC}"
    exit 1
fi

if [ -f "$EXTENSION_DIR/vector--0.5.1.sql" ]; then
    echo -e "${GREEN}✓ Extension SQL file found${NC}"
else
    echo -e "${RED}✗ Extension SQL file not found${NC}"
    exit 1
fi

# Check library files
LIB_DIR=$(pg_config --pkglibdir)
if [ -f "$LIB_DIR/vector.so" ]; then
    echo -e "${GREEN}✓ Shared library found: $LIB_DIR/vector.so${NC}"
else
    echo -e "${RED}✗ Shared library not found${NC}"
    exit 1
fi

echo -e "\n${YELLOW}Step 8: Cleaning up...${NC}"
cd /
rm -rf "$WORK_DIR"
apt-get clean
rm -rf /var/lib/apt/lists/*

echo -e "${GREEN}✓ Cleanup complete${NC}"

echo -e "\n${GREEN}=================================================="
echo -e "✓ pgvector installation complete!"
echo -e "==================================================${NC}"

echo -e "\nNext steps:"
echo -e "1. Restart PostgreSQL: ${YELLOW}systemctl restart postgresql${NC} (or restart Docker container)"
echo -e "2. Run migrations: ${YELLOW}diesel migration run${NC}"
echo -e "3. Verify installation:"
echo -e "   ${YELLOW}psql -d your_database -c \"CREATE EXTENSION vector; SELECT extversion FROM pg_extension WHERE extname='vector';\"${NC}"

exit 0
