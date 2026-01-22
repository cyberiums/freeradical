-- Down migration for schema fixes
-- We can't easily restore the "bad" state, so we just drop the tables.
DROP TABLE IF EXISTS media CASCADE;
DROP TABLE IF EXISTS ai_provider_configs CASCADE;
