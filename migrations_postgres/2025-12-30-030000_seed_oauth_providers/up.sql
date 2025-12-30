INSERT INTO oauth_providers (name, client_id, client_secret, enabled)
VALUES 
    ('google', 'your-client-id', 'your-client-secret', true)
ON CONFLICT DO NOTHING;
