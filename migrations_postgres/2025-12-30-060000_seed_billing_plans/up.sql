-- Seed Site Plans (CMS)
INSERT INTO billing_plans (code, name, price_cents, currency, billing_interval, limits, is_active) VALUES
('site_starter_monthly', 'Starter Site', 2500, 'USD', 'month', '{"sites": 1, "visits": 25000, "storage_gb": 10, "bandwidth_gb": 50, "type": "site"}', true),
('site_pro_monthly', 'Professional Site', 5000, 'USD', 'month', '{"sites": 3, "visits": 75000, "storage_gb": 15, "bandwidth_gb": 150, "type": "site"}', true),
('site_growth_monthly', 'Growth Site', 9600, 'USD', 'month', '{"sites": 10, "visits": 100000, "storage_gb": 20, "bandwidth_gb": 200, "type": "site"}', true),
('site_scale_monthly', 'Scale Site', 24200, 'USD', 'month', '{"sites": 30, "visits": 400000, "storage_gb": 50, "bandwidth_gb": 500, "type": "site"}', true),
('site_enterprise_monthly', 'Enterprise Site', 40000, 'USD', 'month', '{"sites": -1, "visits": -1, "storage_gb": -1, "bandwidth_gb": -1, "type": "site"}', true);

-- Seed Store Plans (Commerce)
INSERT INTO billing_plans (code, name, price_cents, currency, billing_interval, limits, is_active) VALUES
('store_startup_monthly', 'Startup Store', 4500, 'USD', 'month', '{"stores": 1, "visits": 25000, "storage_gb": 15, "bandwidth_gb": 75, "features": ["1-click_setup", "daily_backups"], "type": "store"}', true),
('store_pro_monthly', 'Professional Store', 7500, 'USD', 'month', '{"stores": 3, "visits": 75000, "storage_gb": 20, "bandwidth_gb": 150, "features": ["phone_support", "plugin_updates"], "type": "store"}', true),
('store_growth_monthly', 'Growth Store', 13500, 'USD', 'month', '{"stores": 10, "visits": 100000, "storage_gb": 30, "bandwidth_gb": 250, "features": ["abandoned_cart", "multisite"], "type": "store"}', true),
('store_scale_monthly', 'Scale Store', 31000, 'USD', 'month', '{"stores": 30, "visits": 400000, "storage_gb": 60, "bandwidth_gb": 600, "features": ["priority_support", "enhanced_resources"], "type": "store"}', true),
('store_enterprise_monthly', 'Enterprise Store', 60000, 'USD', 'month', '{"stores": -1, "visits": -1, "storage_gb": -1, "bandwidth_gb": -1, "features": ["high_availability", "white_glove"], "type": "store"}', true);
