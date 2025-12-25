-- Rollback search indexes

ALTER TABLE media DROP INDEX idx_media_fulltext;
ALTER TABLE modules DROP INDEX idx_modules_fulltext;
ALTER TABLE pages DROP INDEX idx_pages_fulltext;
