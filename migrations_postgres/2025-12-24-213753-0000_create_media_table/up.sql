-- Create media table for media library
-- Iteration 4, Task 1

CREATE TABLE media (
    id INTEGERSERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINTEGERNOT NULL ,
    width INTEGER,
    height INTEGER,
    folder VARCHAR(255) DEFAULT '/' ,
    storage_path VARCHAR(500) NOT NULL ,
    cdn_url VARCHAR(500) ,
    upload_user_id INTEGER,
    alt_text VARCHAR(255) ,
    caption TEXT ,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    INDEX idx_uuid (uuid),
    INDEX idx_folder (folder),
    INDEX idx_mime_type (mime_type),
    INDEX idx_created_at (created_at)
)  ;

-- Create media variants table (for different sizes/formats)
CREATE TABLE media_variants (
    id INTEGERSERIAL PRIMARY KEY,
    media_id INTEGERNOT NULL,
    variant_name VARCHAR(50) NOT NULL ,
    file_path VARCHAR(500) NOT NULL,
    width INTEGER
    height INTEGER
    file_size BIGINTEGER
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    INDEX idx_media_variant (media_id, variant_name)
)  ;
