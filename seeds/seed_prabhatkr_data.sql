-- =====================================================
-- FreeRadical CMS - Database Seed Script (SIMPLIFIED)
-- User: prabhatkr@gmail.com
-- Purpose: Populate database with dummy data
-- =====================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- 1. GET EXISTING USER
-- =====================================================
DO $$
DECLARE
    v_user_uuid VARCHAR(255);
    v_user_id INTEGER;
BEGIN
    -- Get existing user by username (email is not a field in users table)
    SELECT uuid, id INTO v_user_uuid, v_user_id 
    FROM users 
    WHERE username = 'prabhatkr@gmail.com'  -- Username is stored as email
    LIMIT 1;

    -- If user doesn't exist, try other possible usernames
    IF v_user_uuid IS NULL THEN
        SELECT uuid, id INTO v_user_uuid, v_user_id 
        FROM users 
        WHERE username LIKE '%prabhat%'
        LIMIT 1;
    END IF;

    -- If still not found, throw error
    IF v_user_uuid IS NULL THEN
        RAISE EXCEPTION 'User with prabhat in username not found in database!';
    ELSE
        RAISE NOTICE 'Found user with UUID: % and ID: %', v_user_uuid, v_user_id;
    END IF;

    -- Store for later use
    CREATE TEMP TABLE IF NOT EXISTS _seed_user (uuid VARCHAR(255), id INTEGER);
    DELETE FROM _seed_user;
    INSERT INTO _seed_user VALUES (v_user_uuid, v_user_id);
END $$;

-- =====================================================
-- 2. CLEANUP EXISTING SEED DATA
-- =====================================================
DO $$
DECLARE
    v_user_uuid VARCHAR(255);
BEGIN
    SELECT uuid INTO v_user_uuid FROM _seed_user;

    RAISE NOTICE 'Cleaning up existing seed data...';

    -- Delete existing dummy data (keeping user account)
    DELETE FROM order_items WHERE order_id IN (
        SELECT id FROM orders WHERE user_uuid = v_user_uuid
    );
    DELETE FROM orders WHERE user_uuid = v_user_uuid;
    
    -- Delete CRM data if exists
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'crm_notes') THEN
        DELETE FROM crm_notes WHERE customer_id IN (
            SELECT id FROM crm_customers WHERE user_id = (SELECT id FROM _seed_user)
        );
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'crm_interactions') THEN
        DELETE FROM crm_interactions WHERE customer_id IN (
            SELECT id FROM crm_customers WHERE user_id = (SELECT id FROM _seed_user)
        );
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'crm_customers') THEN
        DELETE FROM crm_customers WHERE user_id = (SELECT id FROM _seed_user);
    END IF;

    -- Note: Not deleting pages/products as they might be shared
    
    RAISE NOTICE 'Cleanup complete';
END $$;

-- =====================================================
-- 3. PAGES (Content)
-- =====================================================
DO $$
DECLARE
    v_user_uuid VARCHAR(255);
    v_page_uuid VARCHAR(255);
BEGIN
    SELECT uuid INTO v_user_uuid FROM _seed_user;

    -- Insert 5 pages
    FOR i IN 1..5 LOOP
        v_page_uuid := uuid_generate_v4()::varchar;
        
        INSERT INTO pages (uuid, page_name, page_url, page_title, time_created)
        VALUES (
            v_page_uuid,
            CASE i
                WHEN 1 THEN 'home'
                WHEN 2 THEN 'about'
                WHEN 3 THEN 'blog'
                WHEN 4 THEN 'contact'
                ELSE 'pricing'
            END,
            CASE i
                WHEN 1 THEN '/'
                WHEN 2 THEN '/about'
                WHEN 3 THEN '/blog'
                WHEN 4 THEN '/contact'
                ELSE '/pricing'
            END,
            CASE i
                WHEN 1 THEN 'Welcome Home'
                WHEN 2 THEN 'About Us'
                WHEN 3 THEN 'Blog Posts'
                WHEN 4 THEN 'Contact Us'
                ELSE 'Pricing Plans'
            END,
            NOW() - (i || ' days')::INTERVAL
        )
        ON CONFLICT (uuid) DO NOTHING;
    END LOOP;

    RAISE NOTICE 'Created 5 pages';
END $$;

-- =====================================================
-- 4. PRODUCTS
-- =====================================================
DO $$
BEGIN
    INSERT INTO products (uuid, name, description, price_cents, sku, inventory_count, is_active, created_at)
    VALUES
        (uuid_generate_v4()::varchar, 'Premium Plan', 'Full-featured premium subscription', 9900, 'PLAN-PREM-001', 999, true, NOW() - INTERVAL '60 days'),
        (uuid_generate_v4()::varchar, 'Enterprise Plan', 'Enterprise solution with support', 29900, 'PLAN-ENT-001', 999, true, NOW() - INTERVAL '50 days'),
        (uuid_generate_v4()::varchar, 'Developer Plan', 'Perfect for developers', 14900, 'PLAN-DEV-001', 999, true, NOW() - INTERVAL '40 days'),
        (uuid_generate_v4()::varchar, 'Starter Plan', 'Get started with basics', 4900, 'PLAN-START-001', 999, true, NOW() - INTERVAL '30 days'),
        (uuid_generate_v4()::varchar, 'Priority Support Add-on', 'Priority support', 1900, 'ADDON-SUP-001', 999, true, NOW() - INTERVAL '20 days')
    ON CONFLICT (sku) DO NOTHING;

    RAISE NOTICE 'Created 5 products';
END $$;

-- =====================================================
-- 5. ORDERS
-- =====================================================
DO $$
DECLARE
    v_user_uuid VARCHAR(255);
    v_order_id BIGINT;
    v_product_id BIGINT;
BEGIN
    SELECT uuid INTO v_user_uuid FROM _seed_user;

    -- Create 5 orders
    FOR i IN 1..5 LOOP
        INSERT INTO orders (uuid, user_uuid, total_amount_cents, status, payment_provider, created_at)
        VALUES (
            uuid_generate_v4()::varchar,
            v_user_uuid,
            CASE i
                WHEN 1 THEN 9900
                WHEN 2 THEN 29900
                WHEN 3 THEN 14900
                WHEN 4 THEN 4900
                ELSE 11800
            END,
            CASE i
                WHEN 1 THEN 'completed'
                WHEN 2 THEN 'completed'
                WHEN 3 THEN 'processing'
                WHEN 4 THEN 'pending'
                ELSE 'completed'
            END,
            CASE i % 2
                WHEN 0 THEN 'stripe'
                ELSE 'paypal'
            END,
            NOW() - (i * 5 || ' days')::INTERVAL
        )
        ON CONFLICT DO NOTHING;
    END LOOP;

    RAISE NOTICE 'Created 5 orders';
END $$;

-- =====================================================
-- 6. CRM CUSTOMER (if table exists)
-- =====================================================
DO $$
DECLARE
    v_user_id INTEGER;
    v_customer_id INTEGER;
BEGIN
    SELECT id INTO v_user_id FROM _seed_user;

    -- Only insert if crm_customers table exists
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'crm_customers') THEN
        INSERT INTO crm_customers (
            user_id,
            lifecycle_stage,
            customer_since,
            last_purchase_date,
            rfm_recency_score,
            rfm_frequency_score,
            rfm_monetary_score,
            total_orders,
            total_revenue,
            average_order_value,
            customer_lifetime_value,
            health_score,
            churn_risk,
            tags
        )
        VALUES (
            v_user_id,
            'customer',
            NOW() - INTERVAL '90 days',
            NOW() - INTERVAL '2 days',
            5, 4, 5,
            5,
            714.00,
            142.80,
            750.00,
            92,
            'low',
            ARRAY['premium', 'engaged']
        )
        ON CONFLICT (user_id) DO UPDATE SET
            total_orders = EXCLUDED.total_orders,
            health_score = EXCLUDED.health_score;

        RAISE NOTICE 'Created CRM customer profile';
    END IF;
END $$;

-- =====================================================
-- 7. TENANTS (if table exists)
-- =====================================================
DO $$
DECLARE
    v_user_uuid VARCHAR(255);
BEGIN
    SELECT uuid INTO v_user_uuid FROM _seed_user;

    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'tenants') THEN
        INSERT INTO tenants (uuid, name, subdomain, custom_domain, plan, is_active, created_at)
        VALUES
            (uuid_generate_v4()::varchar, 'Main Website', 'main', 'example.com', 'premium', true, NOW() - INTERVAL '60 days'),
            (uuid_generate_v4()::varchar, 'Blog Platform', 'blog', 'blog.example.com', 'premium', true, NOW() - INTERVAL '50 days'),
            (uuid_generate_v4()::varchar, 'Documentation Site', 'docs', 'docs.example.com', 'premium', true, NOW() - INTERVAL '40 days'),
            (uuid_generate_v4()::varchar, 'Support Portal', 'support', 'support.example.com', 'premium', true, NOW() - INTERVAL '30 days'),
            (uuid_generate_v4()::varchar, 'Community Forum', 'community', 'community.example.com', 'premium', true, NOW() - INTERVAL '20 days')
        ON CONFLICT (subdomain) DO NOTHING;

        RAISE NOTICE 'Created 5 tenants';
    END IF;
END $$;

-- =====================================================
-- 8. ADDITIONAL TEAM MEMBERS
-- =====================================================
DO $$
BEGIN
    INSERT INTO users (uuid, username, password)
    VALUES
        (uuid_generate_v4()::varchar, 'sarah_johnson', '$2b$12$hash1'),
        (uuid_generate_v4()::varchar, 'michael_chen', '$2b$12$hash2'),
        (uuid_generate_v4()::varchar, 'emma_rodriguez', '$2b$12$hash3'),
        (uuid_generate_v4()::varchar, 'james_wilson', '$2b$12$hash4'),
        (uuid_generate_v4()::varchar, 'lisa_anderson', '$2b$12$hash5')
    ON CONFLICT (username) DO NOTHING;

    RAISE NOTICE 'Created 5 team members';
END $$;

-- =====================================================
-- CLEANUP
-- =====================================================
DROP TABLE IF EXISTS _seed_user;

-- =====================================================
-- SUMMARY
-- =====================================================
DO $$
BEGIN
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Seed completed for prabhatkr@gmail.com';
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Total Pages: %', (SELECT COUNT(*) FROM pages);
    RAISE NOTICE 'Total Products: %', (SELECT COUNT(*) FROM products);
    RAISE NOTICE 'Total Orders: %', (SELECT COUNT(*) FROM orders);
    RAISE NOTICE 'Total Users: %', (SELECT COUNT(*) FROM users);
    RAISE NOTICE '========================================';
END $$;
