-- Create media table for media library
-- Iteration 4, Task 1

CREATE TABLE media (
    id INT AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL COMMENT 'Size in bytes',
    width INT COMMENT 'Image/video width in pixels',
    height INT COMMENT 'Image/video height in pixels',
    folder VARCHAR(255) DEFAULT '/' COMMENT 'Organization folder path',
    storage_path VARCHAR(500) NOT NULL COMMENT 'Actual file path on disk',
    cdn_url VARCHAR(500) COMMENT 'CDN URL if using CDN',
    upload_user_id INT COMMENT 'User who uploaded',
    alt_text VARCHAR(255) COMMENT 'Alt text for images',
    caption TEXT COMMENT 'Media caption',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    INDEX idx_uuid (uuid),
    INDEX idx_folder (folder),
    INDEX idx_mime_type (mime_type),
    INDEX idx_created_at (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Create media variants table (for different sizes/formats)
CREATE TABLE media_variants (
    id INT AUTO_INCREMENT PRIMARY KEY,
    media_id INT NOT NULL,
    variant_name VARCHAR(50) NOT NULL COMMENT 'thumbnail, medium, large, webp, etc',
    file_path VARCHAR(500) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    INDEX idx_media_variant (media_id, variant_name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
