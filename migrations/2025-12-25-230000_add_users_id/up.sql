-- Add integer ID to users table for foreign key references
ALTER TABLE users ADD COLUMN IF NOT EXISTS id SERIAL;
CREATE UNIQUE INDEX IF NOT EXISTS users_id_idx ON users(id);
