-- Add full-text search indexes

-- Pages full-text index
ALTER TABLE pages
ADD FULLTEXT INDEX idx_pages_fulltext (page_title, page_name, meta_title, meta_description);

-- Modules full-text index
ALTER TABLE modules
ADD FULLTEXT INDEX idx_modules_fulltext (title, content);

-- Media full-text index  
ALTER TABLE media
ADD FULLTEXT INDEX idx_media_fulltext (filename, original_filename, alt_text, caption);
