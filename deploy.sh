#!/bin/bash
set -e

# Oxidly Cloud Platform - Deployment Script

echo "🚀 Starting Deployment..."

# 1. Update Code
echo "📦 Pulling latest code..."
git pull origin main || echo "⚠️ Git pull failed, continuing with local changes..."

# 2. Check Environment
if [ ! -f .env ]; then
    echo "❌ .env file missing! Please copy .env.example to .env and configure it."
    exit 1
fi

# 3. Build Containers
echo "🏗️ Building Docker containers..."
docker-compose build

# 4. Migrate Database (if running)
# Assumes cms container is up or will be up. 
# For zero-downtime, migration strategies differ, but here we restart.

# 5. Restart Services
echo "🔄 Restarting services..."
docker-compose down
docker-compose up -d

# 6. Verify Health
echo "🏥 Checking health..."
echo "Waiting for CMS..."
./scripts/wait-for-it.sh localhost:8000 --timeout=30 -- echo "✅ CMS is up"
echo "Waiting for Oxidly..."
./scripts/wait-for-it.sh localhost:3000 --timeout=30 -- echo "✅ Oxidly is up"

echo "🎉 Deployment Complete!"
