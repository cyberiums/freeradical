-- Create page revisions table for version control
-- Iteration 4, Task 2

CREATE TABLE IF NOT EXISTS page_revisions (
    id BIGSERIAL PRIMARY KEY,
    page_uuid VARCHAR(36) NOT NULL,
    revision_number INT NOT NULL,
    page_title VARCHAR(255) NOT NULL,
    page_url VARCHAR(500) NOT NULL,
    page_content TEXT,
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords VARCHAR(255),
    canonical_url VARCHAR(500),
    full_snapshot JSON NOT NULL, -- Complete page state snapshot
    change_summary VARCHAR(500), -- Brief description of changes
    changed_by_user_id INT, -- User who made the change
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_page_uuid ON page_revisions(page_uuid);
CREATE INDEX IF NOT EXISTS idx_revision ON page_revisions(page_uuid, revision_number);
CREATE INDEX IF NOT EXISTS idx_created_at ON page_revisions(created_at);
CREATE INDEX IF NOT EXISTS idx_user ON page_revisions(changed_by_user_id);

-- Add revision tracking to pages table
ALTER TABLE pages 
ADD COLUMN IF NOT EXISTS current_revision INT DEFAULT 1, -- Current revision number
ADD COLUMN IF NOT EXISTS last_modified_by INT; -- Last user to modify
