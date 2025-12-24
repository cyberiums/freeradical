-- Add scheduled publishing fields to pages
-- Iteration 4, Task 4

ALTER TABLE pages
ADD COLUMN status ENUM('draft', 'scheduled', 'published', 'archived') 
    DEFAULT 'published' 
    COMMENT 'Publication status',
ADD COLUMN publish_at TIMESTAMP NULL 
    COMMENT 'Scheduled publish time',
ADD COLUMN unpublish_at TIMESTAMP NULL 
    COMMENT 'Scheduled unpublish time',
ADD INDEX idx_status (status),
ADD INDEX idx_publish_at (publish_at),
ADD INDEX idx_scheduled (status, publish_at);
