-- Add SEO fields to pages table
ALTER TABLE pages
ADD COLUMN meta_title VARCHAR(70) NULL COMMENT 'SEO-optimized title tag',
ADD COLUMN meta_description VARCHAR(160) NULL COMMENT 'Meta description for search engines',
ADD COLUMN meta_keywords VARCHAR(255) NULL COMMENT 'Legacy meta keywords',
ADD COLUMN canonical_url VARCHAR(500) NULL COMMENT 'Canonical URL for this page',
ADD COLUMN og_title VARCHAR(70) NULL COMMENT 'Open Graph title',
ADD COLUMN og_description VARCHAR(200) NULL COMMENT 'Open Graph description',
ADD COLUMN og_image VARCHAR(500) NULL COMMENT 'Open Graph image URL',
ADD COLUMN twitter_card VARCHAR(20) DEFAULT 'summary' COMMENT 'Twitter card type',
ADD COLUMN twitter_title VARCHAR(70) NULL COMMENT 'Twitter card title',
ADD COLUMN twitter_description VARCHAR(200) NULL COMMENT 'Twitter card description';
