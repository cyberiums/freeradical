-- Add composite indexes for common query patterns
-- Phase 2 query optimization

-- Composite index for common page + module queries
CREATE INDEX IF NOT EXISTS idx_modules_page_category ON modules(page_uuid, category_uuid);

-- Composite index for active pages with recent updates
CREATE INDEX IF NOT EXISTS idx_pages_time_url ON pages(time_created DESC, page_url);

-- Note: Individual indexes already exist from Phase 1:
-- - idx_pages_page_url
-- - idx_pages_time_created  
-- - idx_modules_page_uuid
-- - idx_modules_category_uuid
-- - idx_module_category_page_uuid

-- These composite indexes complement existing ones for specific query patterns
