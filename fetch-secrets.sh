#!/bin/bash

# Fetch secrets from GCP Secret Manager and create .env.production file
# Usage: ./fetch-secrets.sh

set -e

echo "🔐 Fetching secrets from GCP Secret Manager..."

# Check if gcloud is authenticated
if ! gcloud auth list --filter=status:ACTIVE --format="value(account)" &>/dev/null; then
    echo "❌ Error: gcloud is not authenticated"
    echo "Run: gcloud auth login"
    exit 1
fi

# Fetch secrets and create .env file
cat > .env.production << EOF
# Auto-generated from GCP Secret Manager
# Generated at: $(date)
# DO NOT EDIT MANUALLY - Run ./fetch-secrets.sh to refresh

CLOUD_SQL_CONNECTION_NAME=$(gcloud secrets versions access latest --secret=cloud-sql-connection 2>/dev/null || echo "ERROR: cloud-sql-connection secret not found")
POSTGRES_USER=freeradical
POSTGRES_PASSWORD=$(gcloud secrets versions access latest --secret=db-password 2>/dev/null || echo "ERROR: db-password secret not found")
POSTGRES_DB=freeradical
JWT_SECRET=$(gcloud secrets versions access latest --secret=jwt-secret 2>/dev/null || echo "ERROR: jwt-secret secret not found")
APP_BASE_URL=https://your-domain.com
RUST_LOG=info
GOOGLE_CLIENT_ID=$(gcloud secrets versions access latest --secret=google-client-id 2>/dev/null || echo "ERROR: google-client-id secret not found")
GOOGLE_CLIENT_SECRET=$(gcloud secrets versions access latest --secret=google-client-secret 2>/dev/null || echo "ERROR: google-client-secret secret not found")
GOOGLE_REDIRECT_URI=https://your-domain.com/oauth/callback
EOF

# Check for errors
if grep -q "ERROR:" .env.production; then
    echo "❌ Some secrets could not be fetched. Check the errors above."
    echo "💡 Create missing secrets with:"
    echo "   echo -n 'your-secret-value' | gcloud secrets create secret-name --data-file=-"
    exit 1
fi

echo "✅ Secrets fetched successfully and written to .env.production"
echo "🔒 File permissions: $(ls -la .env.production | awk '{print $1}')"

# Secure the file
chmod 600 .env.production

echo "✅ Ready to deploy with: docker-compose -f docker-compose.production.yml up -d"
