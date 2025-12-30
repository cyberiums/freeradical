-- Create media table for media library
-- Iteration 4, Task 1

CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,
    width INTEGER,
    height INTEGER,
    folder VARCHAR(255) DEFAULT '/',
    storage_path VARCHAR(500) NOT NULL,
    cdn_url VARCHAR(500),
    upload_user_id INTEGER,
    alt_text VARCHAR(255),
    caption TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_media_uuid ON media (uuid);
CREATE INDEX idx_media_folder ON media (folder);
CREATE INDEX idx_media_mime_type ON media (mime_type);
CREATE INDEX idx_media_created_at ON media (created_at);

SELECT diesel_manage_updated_at('media');

-- Create media variants table (for different sizes/formats)
CREATE TABLE media_variants (
    id SERIAL PRIMARY KEY,
    media_id INTEGER NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    variant_name VARCHAR(50) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    width INTEGER,
    height INTEGER,
    file_size BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_media_id_variant ON media_variants (media_id, variant_name);
