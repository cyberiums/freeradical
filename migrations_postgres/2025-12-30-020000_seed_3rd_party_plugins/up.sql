-- Seed 3rd Party Plugins (Marketplace)
-- Using developer_id 1 (root) for simulation

INSERT INTO marketplace_plugins (
    name, description, version, file_path, icon_url, status, developer_id, price_cents, downloads_count, created_at
) VALUES 
('Stripe Connect', 'Accept payments via Stripe Connect.', '1.0.0', 'plugins/stripe.zip', 'https://oxidly.com/icons/stripe.png', 'approved', 1, 0, 45, NOW()),
('Mailchimp Sync', 'Sync contacts with Mailchimp.', '2.0.0', 'plugins/mailchimp.zip', 'https://oxidly.com/icons/mailchimp.png', 'approved', 1, 900, 30, NOW()),
('Zapier Integration', 'Connect to 3000+ apps via Zapier.', '1.5.0', 'plugins/zapier.zip', 'https://oxidly.com/icons/zapier.png', 'approved', 1, 0, 150, NOW())
ON CONFLICT DO NOTHING;
