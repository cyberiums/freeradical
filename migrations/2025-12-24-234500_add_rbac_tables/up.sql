-- Create RBAC (Role-Based Access Control) tables

-- Roles table
CREATE TABLE roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    permissions JSON COMMENT 'Array of permission strings like ["pages.*", "media.read"]',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- User-Role mapping (many-to-many)
CREATE TABLE user_roles (
    user_id VARCHAR(255) NOT NULL,
    role_id INT NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    assigned_by VARCHAR(255),
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(uuid) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

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
