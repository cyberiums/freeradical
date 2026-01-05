#!/bin/bash
# Seed database with dummy data for prabhatkr@gmail.com

set -e

echo "========================================="
echo "Seeding FreeRadical Database"
echo "========================================="

# Check if running in Docker
if [ -f /.dockerenv ]; then
    echo "Running inside Docker container..."
    psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f /app/seeds/seed_prabhatkr_data.sql
else
    echo "Running from host..."
    docker-compose exec -T cms psql -U freeradical -d freeradical_db < seeds/seed_prabhatkr_data.sql
fi

echo "========================================="
echo "✅ Database seeded successfully!"
echo "========================================="
