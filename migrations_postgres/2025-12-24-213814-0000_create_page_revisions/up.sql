-- Create page revisions table for version control
-- Iteration 4, Task 2

CREATE TABLE page_revisions (
    id BIGINTEGERSERIAL PRIMARY KEY,
    page_uuid VARCHAR(36) NOT NULL,
    revision_number INTEGERNOT NULL,
    page_title VARCHAR(255) NOT NULL,
    page_url VARCHAR(500) NOT NULL,
    page_content TEXT,
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords VARCHAR(255),
    canonical_url VARCHAR(500),
    -- Store full page state as JSON
    full_snapshot JSON NOT NULL ,
    change_summary VARCHAR(500) ,
    changed_by_user_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_page_uuid (page_uuid),
    INDEX idx_revision (page_uuid, revision_number),
    INDEX idx_created_at (created_at),
    INDEX idx_user (changed_by_user_id)
)  ;

-- Add revision tracking to pages table
ALTER TABLE pages 
ADD COLUMN current_revision INTEGERDEFAULT 1 ,
ADD COLUMN last_modified_by INTEGER;
