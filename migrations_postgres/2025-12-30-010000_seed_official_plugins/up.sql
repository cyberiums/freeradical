-- Seed Official Plugins (Marketplace)
-- developer_id 1 is 'root'

INSERT INTO marketplace_plugins (
    name, description, version, file_path, icon_url, status, developer_id, price_cents, downloads_count, created_at
) VALUES 
('SEO Pro', 'Advanced SEO tools including sitemaps, meta tags, and analyzing content.', '1.0.0', 'plugins/seo_pro.zip', 'https://oxidly.com/icons/seo.png', 'approved', 1, 0, 120, NOW()),
('E-commerce Essentials', 'Core e-commerce features: Cart, Checkout, Inventory.', '2.1.0', 'plugins/ecommerce_core.zip', 'https://oxidly.com/icons/cart.png', 'approved', 1, 0, 500, NOW()),
('Analytics Dashboard', 'Visual analytics and reporting dashboard.', '1.2.0', 'plugins/analytics.zip', 'https://oxidly.com/icons/chart.png', 'approved', 1, 2900, 85, NOW()),
('Advanced Forms', 'Drag-and-drop form builder with integrations.', '1.0.5', 'plugins/forms.zip', 'https://oxidly.com/icons/form.png', 'approved', 1, 1900, 200, NOW()),
('Newsletter Manager', 'Email marketing and newsletter management.', '1.1.0', 'plugins/newsletter.zip', 'https://oxidly.com/icons/mail.png', 'approved', 1, 0, 310, NOW())
ON CONFLICT DO NOTHING;
