-- Add full-text search indexes using GIN and tsvector

-- Pages full-text index
CREATE INDEX idx_pages_fulltext ON pages USING GIN (to_tsvector('english', 
    coalesce(page_title,'') || ' ' || 
    coalesce(page_name,'') || ' ' || 
    coalesce(meta_title,'') || ' ' || 
    coalesce(meta_description,'')
));

-- Modules full-text index
CREATE INDEX idx_modules_fulltext ON modules USING GIN (to_tsvector('english', 
    coalesce(title,'') || ' ' || 
    coalesce(content,'')
));

-- Media full-text index
CREATE INDEX idx_media_fulltext ON media USING GIN (to_tsvector('english', 
    coalesce(filename,'') || ' ' || 
    coalesce(original_filename,'') || ' ' || 
    coalesce(alt_text,'') || ' ' || 
    coalesce(caption,'')
));
