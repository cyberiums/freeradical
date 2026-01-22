-- Add content field to pages table for storing HTML content
ALTER TABLE pages ADD COLUMN content TEXT;

-- Add comment describing the field
COMMENT ON COLUMN pages.content IS 'HTML content of the page, stored from rich text editor';
