-- Undo Seed Official Plugins

DELETE FROM marketplace_plugins WHERE name IN (
    'SEO Pro', 'E-commerce Essentials', 'Analytics Dashboard', 'Advanced Forms', 'Newsletter Manager'
);
