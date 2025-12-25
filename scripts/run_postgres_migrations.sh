#!/bin/bash
# Consolidated PostgreSQL migration runner
set -e

POSTGRES_USER=${POSTGRES_USER:-freeradical}
POSTGRES_DB=${POSTGRES_DB:-freeradical}

echo "📊 Running all PostgreSQL migrations via SQL..."

# Consolidate all migrations into a single SQL file for simplicity
cat > /tmp/postgres_all_migrations.sql << 'EOSQL'
-- Combined PostgreSQL Migrations for FreeRadical CMS

-- Create users table first
CREATE TABLE IF NOT EXISTS users (
    id SERIAL,
    uuid varchar(255) PRIMARY KEY,
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    token varchar(511),
    two_factor_secret varchar(255),
    two_factor_enabled BOOLEAN DEFAULT FALSE NOT NULL
);

-- Create pages table
CREATE TABLE IF NOT EXISTS pages (
    uuid varchar(255) PRIMARY KEY,
    page_name varchar(500) NOT NULL,
    page_url varchar(255) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    meta_title varchar(70),
    meta_description varchar(160),
    meta_keywords varchar(255),
    canonical_url varchar(500),
    og_title varchar(70),
    og_description varchar(200),
    og_image varchar(500),
    twitter_card varchar(20),
    twitter_title varchar(70),
    twitter_description varchar(200),
    author varchar(100),
    article_type varchar(50),
    featured_image varchar(500),
    word_count INTEGER,
    reading_time INTEGER,
    current_revision INTEGER,
    last_modified_by INTEGER,
    status VARCHAR(9) DEFAULT 'published' CHECK (status IN ('draft', 'scheduled', 'published', 'archived')),
    publish_at TIMESTAMP,
    unpublish_at TIMESTAMP
);

-- Create module_category table
CREATE TABLE IF NOT EXISTS module_category (
    uuid varchar(255) PRIMARY KEY,
    page_uuid varchar(255) NOT NULL,
    title varchar(255) NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE
);

-- Create modules table
CREATE TABLE IF NOT EXISTS modules (
    uuid varchar(255) PRIMARY KEY,
    page_uuid VARCHAR(255) NOT NULL,
    category_uuid VARCHAR(255),
    title varchar(255) NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE,
    FOREIGN KEY (category_uuid) REFERENCES module_category(uuid) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_page_url ON pages(page_url);
CREATE INDEX IF NOT EXISTS idx_status ON pages(status);
CREATE INDEX IF NOT EXISTS idx_publish_at ON pages(publish_at);

-- Additional tables (simplified for benchmark)
CREATE TABLE IF NOT EXISTS page_revisions (
    id BIGSERIAL PRIMARY KEY,
    page_uuid varchar(36) NOT NULL,
    revision_number INTEGER NOT NULL,
    page_title varchar(255) NOT NULL,
    page_url varchar(500) NOT NULL,
    page_content TEXT,
    meta_title varchar(70),
    meta_description varchar(160),
    meta_keywords varchar(255),
    canonical_url varchar(500),
    full_snapshot TEXT NOT NULL,
    change_summary varchar(500),
    changed_by_user_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name varchar(50) NOT NULL,
    description TEXT,
    permissions JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id varchar(255) NOT NULL,
    role_id INTEGER NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    assigned_by varchar(255),
    PRIMARY KEY (user_id, role_id)
);

-- Insert seed data
INSERT INTO users (uuid, username, password) 
VALUES (gen_random_uuid()::text, 'root', '')
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO pages (page_name, uuid, page_url, page_title) 
VALUES ('index', gen_random_uuid()::text, '/', 'Home')
ON CONFLICT (uuid) DO NOTHING;

EOSQL

echo "Applying consolidated migrations..."
docker-compose -f docker-compose.postgres.yml exec -T postgres \
    psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < /tmp/postgres_all_migrations.sql

echo "✅ PostgreSQL migrations complete!"
echo "Verifying tables..."
docker-compose -f docker-compose.postgres.yml exec -T postgres \
    psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "\dt" | head -20
