-- Create media table for search service
CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_path TEXT NOT NULL,
    mime_type VARCHAR(127) NOT NULL,
    file_size BIGINT NOT NULL,
    
    -- Image-specific fields
    width INTEGER,
    height INTEGER,
    alt_text TEXT,
    
    -- Metadata
    title VARCHAR(255),
    description TEXT,
    tags TEXT[], -- PostgreSQL array
    
    -- Relationships
    uploaded_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    
    -- Timestamps
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_media_uuid ON media(uuid);
CREATE INDEX idx_media_mime_type ON media(mime_type);
CREATE INDEX idx_media_uploaded_by ON media(uploaded_by) WHERE uploaded_by IS NOT NULL;
CREATE INDEX idx_media_created_at ON media(created_at DESC);
