-- Create page revisions table for version control
-- Iteration 4, Task 2

CREATE TABLE page_revisions (
    id BIGSERIAL PRIMARY KEY,
    page_uuid VARCHAR(36) NOT NULL,
    revision_number INTEGER NOT NULL,
    page_title VARCHAR(255) NOT NULL,
    page_url VARCHAR(500) NOT NULL,
    page_content TEXT,
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords VARCHAR(255),
    canonical_url VARCHAR(500),
    -- Store full page state as JSON (TEXT for compatibility with schema.rs)
    full_snapshot TEXT NOT NULL,
    change_summary VARCHAR(500),
    changed_by_user_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_rev_page_uuid ON page_revisions (page_uuid);
CREATE INDEX idx_rev_revision ON page_revisions (page_uuid, revision_number);
CREATE INDEX idx_rev_created_at ON page_revisions (created_at);
CREATE INDEX idx_rev_user ON page_revisions (changed_by_user_id);

-- Add revision tracking to pages table
ALTER TABLE pages ADD COLUMN IF NOT EXISTS current_revision INTEGER DEFAULT 1;
ALTER TABLE pages ADD COLUMN IF NOT EXISTS last_modified_by INTEGER;
