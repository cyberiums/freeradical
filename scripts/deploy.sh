#!/bin/bash
set -e

echo "🚀 Deploying FreeRadical CMS..."

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ .env file not found. Copy .env.example to .env and configure it."
    exit 1
fi

# Source environment variables
source .env

echo "📦 Building Docker containers..."
docker-compose build

echo "🗄️  Starting MySQL and Redis..."
docker-compose up -d mysql redis

echo "⏳ Waiting for database to be ready..."
sleep 10

echo "🔄 Running database migrations..."
docker-compose run --rm cms diesel migration run

echo "🚀 Starting all services..."
docker-compose up -d

echo "✅ Deployment complete!"
echo ""
echo "Services running:"
echo "  CMS API:    http://localhost:${APP_PORT:-8000}"
echo "  Admin UI:   http://localhost:${ADMIN_PORT:-3000}"
echo "  GraphQL:    http://localhost:${APP_PORT:-8000}/graphql"
echo ""
echo "To view logs: docker-compose logs -f"
echo "To stop:      docker-compose down"
