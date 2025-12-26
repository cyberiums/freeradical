-- Add article-specific metadata fields for Article schema support
DO $$
BEGIN
    ALTER TABLE pages ADD COLUMN IF NOT EXISTS author VARCHAR(100);
    ALTER TABLE pages ADD COLUMN IF NOT EXISTS article_type VARCHAR(50) DEFAULT 'WebPage';
    ALTER TABLE pages ADD COLUMN IF NOT EXISTS featured_image VARCHAR(500);
    ALTER TABLE pages ADD COLUMN IF NOT EXISTS word_count INT;
    ALTER TABLE pages ADD COLUMN IF NOT EXISTS reading_time INT;
END $$;
