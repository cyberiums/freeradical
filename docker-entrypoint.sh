#!/bin/bash
# Docker entrypoint script for FreeRadical CMS
# Runs migrations automatically before starting the application

set -e

echo "🚀 FreeRadical CMS Starting..."
echo ""

# Wait for database to be ready
echo "⏳ Waiting for database connection..."
until pg_isready -h "${DATABASE_HOST:-db}" -U "${DATABASE_USER:-freeradical}" > /dev/null 2>&1; do
  echo "Database not ready, waiting..."
  sleep 2
done
echo "✅ Database connection established"
echo ""

# Install diesel_cli if not present (for migrations)
if ! command -v diesel &> /dev/null; then
    echo "📦 Installing diesel_cli..."
    cargo install diesel_cli --no-default-features --features postgres --quiet || true
fi

# Run migrations
echo "🔄 Running database migrations..."
if [ -d "/app/migrations_postgres" ]; then
    diesel migration run --migration-dir /app/migrations_postgres || {
        echo "❌ Migration failed!"
        echo "ERROR: Database migrations could not be applied."
        echo "This is likely due to:"
        echo "  1. Database connection issues"
        echo "  2. Invalid migration SQL"
        echo "  3. Missing DATABASE_URL environment variable"
        echo ""
        echo "Continuing anyway (migrations may have already been applied)..."
    }
    echo "✅ Migrations applied successfully"
else
    echo "⚠️  No migrations directory found, skipping..."
fi
echo ""

# Start the application
echo "🎯 Starting FreeRadical CMS..."
echo "========================================="
exec "$@"
