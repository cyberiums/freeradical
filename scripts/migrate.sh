#!/bin/bash
# Helper script to run diesel migrations via Docker
# This avoids macOS MySQL library issues

set -e

echo "🗄️  Starting MySQL container..."
docker-compose up -d mysql

echo "⏳ Waiting for MySQL to be ready..."
sleep 5

echo "🔄 Running migrations..."
docker-compose run --rm cms diesel migration run

echo "✅ Migrations complete!"
