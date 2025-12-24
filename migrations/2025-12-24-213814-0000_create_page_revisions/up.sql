-- Create page revisions table for version control
-- Iteration 4, Task 2

CREATE TABLE page_revisions (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    page_uuid VARCHAR(36) NOT NULL,
    revision_number INT NOT NULL,
    page_title VARCHAR(255) NOT NULL,
    page_url VARCHAR(500) NOT NULL,
    page_content TEXT,
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords VARCHAR(255),
    canonical_url VARCHAR(500),
    -- Store full page state as JSON
    full_snapshot JSON NOT NULL COMMENT 'Complete page state snapshot',
    change_summary VARCHAR(500) COMMENT 'Brief description of changes',
    changed_by_user_id INT COMMENT 'User who made the change',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_page_uuid (page_uuid),
    INDEX idx_revision (page_uuid, revision_number),
    INDEX idx_created_at (created_at),
    INDEX idx_user (changed_by_user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Add revision tracking to pages table
ALTER TABLE pages 
ADD COLUMN current_revision INT DEFAULT 1 COMMENT 'Current revision number',
ADD COLUMN last_modified_by INT COMMENT 'Last user to modify';
