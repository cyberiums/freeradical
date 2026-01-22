-- Remove content field from pages table
ALTER TABLE pages DROP COLUMN IF EXISTS content;
