-- Rollback SEO fields from pages table
ALTER TABLE pages
DROP COLUMN meta_title,
DROP COLUMN meta_description,
DROP COLUMN meta_keywords,
DROP COLUMN canonical_url,
DROP COLUMN og_title,
DROP COLUMN og_description,
DROP COLUMN og_image,
DROP COLUMN twitter_card,
DROP COLUMN twitter_title,
DROP COLUMN twitter_description;
