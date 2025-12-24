-- Rollback publishing fields

ALTER TABLE pages 
DROP INDEX idx_scheduled,
DROP INDEX idx_publish_at,
DROP INDEX idx_status,
DROP COLUMN unpublish_at,
DROP COLUMN publish_at,
DROP COLUMN status;
