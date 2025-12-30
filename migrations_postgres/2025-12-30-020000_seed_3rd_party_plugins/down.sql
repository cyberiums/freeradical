-- Undo Seed 3rd Party Plugins

DELETE FROM marketplace_plugins WHERE name IN (
    'Stripe Connect', 'Mailchimp Sync', 'Zapier Integration'
);
