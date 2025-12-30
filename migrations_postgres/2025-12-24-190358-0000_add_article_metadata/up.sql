-- Add article-specific metadata fields for Article schema support
-- These fields enable rich Article/BlogPosting structured data

ALTER TABLE pages
ADD COLUMN author VARCHAR(100),
ADD COLUMN article_type VARCHAR(50) DEFAULT 'WebPage',
ADD COLUMN featured_image VARCHAR(500),
ADD COLUMN word_count INTEGER,
ADD COLUMN reading_time INTEGER

-- Note: publish_date can use existing time_created field
-- article_type: 'Article', 'BlogPosting', 'NewsArticle', or 'WebPage' (default)
