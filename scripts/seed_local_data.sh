#!/bin/bash
set -e

echo "Waiting for postgres..."
until docker-compose exec -T postgres pg_isready -U freeradical; do
  sleep 1
done

echo "Seeding local data..."

docker-compose exec -T postgres psql -U freeradical -d freeradical -c "
-- Seed Default Tenant
INSERT INTO tenants (id, uuid, name, subdomain, is_active, plan, created_at, updated_at)
VALUES (1, 'default-tenant-uuid', 'Default Org', 'default', true, 'enterprise', NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET 
    custom_domain = 'localhost';

-- Seed Admin User (Password is 'password')
-- Note: Password hash should be valid Argon2 hash. 
-- Using a placeholder hash for now or generating one. 
-- For simplicity in dev, we might update password later via API or assume hash is pre-calculated.
INSERT INTO users (id, uuid, username, password, two_factor_enabled)
VALUES (1, 'admin-user-uuid', 'admin@freeradical.dev', '\$argon2id\$v=19\$m=4096,t=3,p=1\$salt\$hashplaceholder', false)
ON CONFLICT (id) DO NOTHING;

-- Seed Tenant Member linking Admin to Default Tenant
INSERT INTO tenant_members (id, tenant_id, user_id, role, status, created_at, updated_at)
VALUES (1, 1, 1, 'owner', 'active', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
"

echo "Seeding complete."
