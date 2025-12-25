-- Add scheduled publishing fields to pages (PostgreSQL version)
-- Iteration 4, Task 4
-- PostgreSQL uses VARCHAR instead of MySQL ENUM

ALTER TABLE pages
ADD COLUMN status VARCHAR(9) 
    DEFAULT 'published'
    CHECK (status IN ('draft', 'scheduled', 'published', 'archived')),
ADD COLUMN publish_at TIMESTAMP NULL,
ADD COLUMN unpublish_at TIMESTAMP NULL;

-- Create indexes
CREATE INDEX idx_status ON pages(status);
CREATE INDEX idx_publish_at ON pages(publish_at);
CREATE INDEX idx_scheduled ON pages(status, publish_at);

-- Add comments (PostgreSQL syntax)
COMMENT ON COLUMN pages.status IS 'Publication status';
COMMENT ON COLUMN pages.publish_at IS 'Scheduled publish time';
COMMENT ON COLUMN pages.unpublish_at IS 'Scheduled unpublish time';
