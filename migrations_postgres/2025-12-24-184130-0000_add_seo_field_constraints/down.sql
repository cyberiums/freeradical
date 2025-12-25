-- Rollback migration: Remove SEO field constraints

ALTER TABLE pages DROP CONSTRAINT chk_meta_title_length;
ALTER TABLE pages DROP CONSTRAINT chk_meta_description_length;
ALTER TABLE pages DROP CONSTRAINT chk_og_title_length;
ALTER TABLE pages DROP CONSTRAINT chk_og_description_length;
ALTER TABLE pages DROP CONSTRAINT chk_twitter_title_length;
ALTER TABLE pages DROP CONSTRAINT chk_twitter_description_length;
ALTER TABLE pages DROP CONSTRAINT chk_canonical_url_format;
ALTER TABLE pages DROP CONSTRAINT chk_og_image_format;
