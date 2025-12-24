-- Rollback migration: Remove performance indexes

DROP INDEX idx_pages_page_url ON pages;
DROP INDEX idx_pages_time_created ON pages;
DROP INDEX idx_modules_page_uuid ON modules;
DROP INDEX idx_modules_category_uuid ON modules;
DROP INDEX idx_module_category_page_uuid ON module_category;
