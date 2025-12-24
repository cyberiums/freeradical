-- Rollback page revisions

ALTER TABLE pages DROP COLUMN current_revision;
ALTER TABLE pages DROP COLUMN last_modified_by;

DROP TABLE page_revisions;
