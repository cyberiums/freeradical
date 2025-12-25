-- Remove publishing fields from pages table
DROP INDEX IF EXISTS idx_scheduled;
DROP INDEX IF EXISTS idx_publish_at;
DROP INDEX IF EXISTS idx_status;

ALTER TABLE pages
DROP COLUMN IF EXISTS unpublish_at,
DROP COLUMN IF EXISTS publish_at,
DROP COLUMN IF EXISTS status;
