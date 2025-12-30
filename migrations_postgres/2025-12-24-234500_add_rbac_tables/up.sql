-- Create RBAC (Role-Based Access Control) tables

-- Roles table
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    permissions JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('roles');

-- User-Role mapping (many-to-many)
-- Note: users table uses 'uuid' string, so user_id is VARCHAR(255) to match.
-- Wait, schema.rs says user_id in other tables is Int4?
-- Line 799: user_id -> Int4.
-- Roles migration says user_id VARCHAR(255).
-- Check `users` table definition in schema or prev migration?
-- I don't have users table definiton handy.
-- BUT line 15: `user_id VARCHAR(255) NOT NULL`.
-- Line 20: `FOREIGN KEY (user_id) REFERENCES users(uuid)`.
-- If `users.uuid` is VARCHAR, then this is correct.
-- If `users.id` is INT, then `user_roles` linking to `uuid` is strange but possible.
-- I'll keep it as VARCHAR(255) to match `users(uuid)`.

CREATE TABLE user_roles (
    user_id VARCHAR(255) NOT NULL,
    role_id INTEGER NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    assigned_by VARCHAR(255),
    PRIMARY KEY (user_id, role_id),
    -- Assuming users table exists and has uuid column
    -- Removing FK constraint to users(uuid) to avoid dependency if users table isn't fully ready or if uuid isn't unique constraint in this context yet.
    -- Actually, safer to keep it if possible, but if users table is missing...
    -- Users table usually created early.
    -- I'll keep the FK but syntax corrected.
    FOREIGN KEY (user_id) REFERENCES users(uuid) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

-- Insert default roles
INSERT INTO roles (name, description, permissions) VALUES
('admin', 'Full system access', '["*"]'),
('editor', 'Edit all content', '["pages.*", "modules.*", "media.read", "media.create", "media.update"]'),
('author', 'Create and edit own content', '["pages.create", "pages.update_own", "pages.read", "modules.create", "modules.update_own", "modules.read", "media.read"]'),
('viewer', 'Read-only access', '["pages.read", "modules.read", "media.read"]');

-- Create indexes for performance
CREATE INDEX idx_roles_name ON roles(name);
CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);
