-- Add performance indexes for FreeRadical CMS
-- Critical indexes for query optimization on frequently accessed columns

-- Index for route matching (most critical - every page request uses this)
CREATE INDEX IF NOT EXISTS idx_pages_page_url ON pages(page_url);

-- Index for sitemap generation and time-based queries
CREATE INDEX IF NOT EXISTS idx_pages_time_created ON pages(time_created DESC);

-- Foreign key indexes for JOIN optimization (modules table)
CREATE INDEX IF NOT EXISTS idx_modules_page_uuid ON modules(page_uuid);
CREATE INDEX IF NOT EXISTS idx_modules_category_uuid ON modules(category_uuid);

-- Foreign key index for module_category JOIN operations
CREATE INDEX IF NOT EXISTS idx_module_category_page_uuid ON module_category(page_uuid);
