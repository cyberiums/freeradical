-- Rollback composite indexes

DROP INDEX idx_modules_page_category ON modules;
DROP INDEX idx_pages_time_url ON pages;
