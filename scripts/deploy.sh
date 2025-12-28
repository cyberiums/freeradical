#!/bin/bash
set -e

echo "🚀 Deploying FreeRadical Cloud Platform (v2.0.0)..."

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ .env file not found. Copy .env.example to .env and configure it."
    exit 1
fi

# Source environment variables
source .env

echo "📦 Building Docker containers (CMS, Oxidly, Admin)..."
docker-compose build

echo "🗄️  Starting Databases..."
# Determine DB service based on env or default to postgres
if [[ $DATABASE_URL == *"mysql"* ]]; then
    echo "   -> Detected MySQL configuration"
    docker-compose up -d mysql redis
else
    echo "   -> Using PostgreSQL (Default)"
    docker-compose up -d postgres redis
fi

echo "⏳ Waiting for database to be ready..."
sleep 10

echo "🔄 Running database migrations..."
docker-compose run --rm cms diesel migration run

echo "🚀 Starting all services..."
docker-compose up -d

echo "✅ Deployment complete!"
echo ""
echo "Services running:"
echo "  Oxidly Cloud: http://localhost:${OXIDLY_PORT:-5005}"
echo "  CMS API:      http://localhost:${APP_PORT:-8000}"
echo "  Admin UI:     http://localhost:${ADMIN_PORT:-3000}"
echo ""
echo "To view logs: docker-compose logs -f"
echo "To stop:      docker-compose down"
