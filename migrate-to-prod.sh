#!/bin/bash
# migrate-to-prod.sh
# Migrates from local docker-compose to production GHCR-based deployment

set -e

echo "========================================="
echo "FreeRadical Production Migration Script"
echo "========================================="
echo ""

# Step 1: Backup current setup
echo "[1/7] Creating backup of current containers..."
docker compose ps > backup_containers_$(date +%Y%m%d_%H%M%S).txt
echo "✅ Backup created"
echo ""

# Step 2: Stop and remove current containers (keep volumes!)
echo "[2/7] Stopping current containers..."
docker compose down
echo "✅ Containers stopped (volumes preserved)"
echo ""

# Step 3: Remove old local images to save space
echo "[3/7] Removing old local images..."
docker rmi freeradical-cms:latest freeradical-oxidly:latest freeradical-admin:latest 2>/dev/null || echo "Local images already removed"
echo "✅ Old images cleaned"
echo ""

# Step 4: Pull latest images from GHCR
echo "[4/7] Pulling latest images from GitHub Container Registry..."
echo "This may take a few minutes..."
docker compose -f docker-compose.prod.yml pull
echo "✅ Images pulled successfully"
echo ""

# Step 5: Start services with new images
echo "[5/7] Starting services with GHCR images..."
docker compose -f docker-compose.prod.yml up -d
echo "✅ Services started"
echo ""

# Step 6: Wait for health checks
echo "[6/7] Waiting for services to be healthy..."
sleep 10
docker compose -f docker-compose.prod.yml ps
echo ""

# Step 7: Verify Watchtower
echo "[7/7] Checking Watchtower configuration..."
docker logs --tail 20 freeradical_watchtower
echo ""

echo "========================================="
echo "✅ Migration Complete!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. Verify services are running: docker compose -f docker-compose.prod.yml ps"
echo "2. Check logs: docker compose -f docker-compose.prod.yml logs -f"
echo "3. Test endpoints:"
echo "   - CMS API: curl http://localhost:8000/health"
echo "   - Oxidly: curl http://localhost:5005/"
echo "   - Admin: curl http://localhost:3000/"
echo ""
echo "Watchtower will now automatically update containers when you push to main branch!"
echo "Monitor updates: docker logs -f freeradical_watchtower"
