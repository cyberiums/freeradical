-- Rollback: Remove article metadata fields

ALTER TABLE pages
DROP COLUMN author,
DROP COLUMN article_type,
DROP COLUMN featured_image,
DROP COLUMN word_count,
DROP COLUMN reading_time;
