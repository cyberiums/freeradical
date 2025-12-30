-- Rollback migration for pending_verifications
DROP INDEX IF EXISTS idx_pending_verifications_verified;
DROP INDEX IF EXISTS idx_pending_verifications_type_email;
DROP INDEX IF EXISTS idx_pending_verifications_expires;
DROP TABLE IF EXISTS pending_verifications;
