-- Seed Development Tenant (localhost)
INSERT INTO tenants (uuid, name, subdomain, custom_domain, plan, is_active, settings, created_at, updated_at)
SELECT 
    '550e8400-e29b-41d4-a716-44665544dev1', 
    'Oxidly Development', 
    'dev', 
    'localhost', 
    'enterprise', 
    true, 
    '{"theme": "default", "currency": "USD"}', 
    NOW(), 
    NOW()
WHERE NOT EXISTS (SELECT 1 FROM tenants WHERE custom_domain = 'localhost');

-- Seed Production Tenant (oxidly.com)
INSERT INTO tenants (uuid, name, subdomain, custom_domain, plan, is_active, settings, created_at, updated_at)
SELECT 
    '550e8400-e29b-41d4-a716-44665544prod', 
    'Oxidly Production', 
    'www', 
    'oxidly.com', 
    'enterprise', 
    true, 
    '{"theme": "default", "currency": "USD"}', 
    NOW(), 
    NOW()
WHERE NOT EXISTS (SELECT 1 FROM tenants WHERE custom_domain = 'oxidly.com');
