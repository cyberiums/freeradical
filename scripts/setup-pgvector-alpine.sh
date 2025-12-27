#!/bin/sh
set -e

echo "=================================================="
echo "pgvector Installation for Alpine Linux/PostgreSQL"
echo "=================================================="

# Configuration
PGVECTOR_VERSION="v0.5.1"

echo "Step 1: Installing build dependencies..."
apk add --no-cache \
    git \
    build-base \
    clang15 \
    llvm15

echo "✓ Dependencies installed"

echo ""
echo "Step 2: Downloading pgvector ${PGVECTOR_VERSION}..."
cd /tmp
rm -rf pgvector
git clone --depth 1 --branch "$PGVECTOR_VERSION" https://github.com/pgvector/pgvector.git
cd pgvector

echo "✓ Downloaded pgvector"

echo ""
echo "Step 3: Building pgvector..."
make clean || true
make

echo "✓ Build successful"

echo ""
echo "Step 4: Installing pgvector..."
make install

echo "✓ pgvector installed"

echo ""
echo "Step 5: Verifying installation..."
EXTENSION_DIR=$(pg_config --sharedir)/extension
if [ -f "$EXTENSION_DIR/vector.control" ]; then
    echo "✓ Extension control file found"
else
    echo "✗ Extension control file not found"
    exit 1
fi

echo ""
echo "Step 6: Cleaning up..."
cd /
rm -rf /tmp/pgvector
apk del git build-base

echo "✓ Cleanup complete"

echo ""
echo "=================================================="
echo "✓ pgvector installation complete!"
echo "=================================================="
echo ""
echo "Next: Restart PostgreSQL or reload configuration"

exit 0
