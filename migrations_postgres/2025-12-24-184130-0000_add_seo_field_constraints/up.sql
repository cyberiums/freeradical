-- Add database-level constraints for SEO fields
-- MySQL 8.0.16+ supports CHECK constraints for additional data validation

-- Add CHECK constraints for SEO field lengths
ALTER TABLE pages
ADD CONSTRAINT chk_meta_title_length 
CHECK (meta_title IS NULL OR CHAR_LENGTH(meta_title) <= 70);

ALTER TABLE pages
ADD CONSTRAINT chk_meta_description_length 
CHECK (meta_description IS NULL OR CHAR_LENGTH(meta_description) <= 160);

ALTER TABLE pages
ADD CONSTRAINT chk_og_title_length 
CHECK (og_title IS NULL OR CHAR_LENGTH(og_title) <= 70);

ALTER TABLE pages
ADD CONSTRAINT chk_og_description_length 
CHECK (og_description IS NULL OR CHAR_LENGTH(og_description) <= 200);

ALTER TABLE pages
ADD CONSTRAINT chk_twitter_title_length 
CHECK (twitter_title IS NULL OR CHAR_LENGTH(twitter_title) <= 70);

ALTER TABLE pages
ADD CONSTRAINT chk_twitter_description_length 
CHECK (twitter_description IS NULL OR CHAR_LENGTH(twitter_description) <= 200);

-- Add CHECK constraint for canonical_url format (must start with http, https, or /)
ALTER TABLE pages
ADD CONSTRAINT chk_canonical_url_format 
CHECK (canonical_url IS NULL OR 
       canonical_url LIKE 'http://%' OR 
       canonical_url LIKE 'https://%' OR 
       canonical_url LIKE '/%');

-- Add CHECK constraint for og_image format (must be valid URL)
ALTER TABLE pages
ADD CONSTRAINT chk_og_image_format 
CHECK (og_image IS NULL OR 
       og_image LIKE 'http://%' OR 
       og_image LIKE 'https://%');
