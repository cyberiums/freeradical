-- Remove integer ID from users table
DROP INDEX IF EXISTS users_id_idx;
ALTER TABLE users DROP COLUMN IF EXISTS id;
