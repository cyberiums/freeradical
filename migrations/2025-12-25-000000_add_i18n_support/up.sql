-- Add multi-language support tables

CREATE TABLE languages (
    id INT PRIMARY KEY AUTO_INCREMENT,
    code VARCHAR(10) UNIQUE NOT NULL COMMENT 'ISO 639-1 code (en, es, fr, de, ja)',
    name VARCHAR(100) NOT NULL COMMENT 'English name',
    native_name VARCHAR(100) COMMENT 'Native name (Español, Français)',
    is_default BOOLEAN DEFAULT FALSE COMMENT 'Default language for site',
    is_rtl BOOLEAN DEFAULT FALSE COMMENT 'Right-to-left language',
    enabled BOOLEAN DEFAULT TRUE COMMENT 'Language is active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE page_translations (
    id INT PRIMARY KEY AUTO_INCREMENT,
    page_id INT NOT NULL,
    language_id INT NOT NULL,
    page_title VARCHAR(255),
    page_content TEXT,
    page_url VARCHAR(255) COMMENT 'Translated slug',
    meta_title VARCHAR(255),
    meta_description TEXT,
    og_title VARCHAR(255),
    og_description TEXT,
    twitter_title VARCHAR(255),
    twitter_description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE,
    UNIQUE KEY unique_page_language (page_id, language_id),
    INDEX idx_language (language_id),
    INDEX idx_page (page_id)
);

CREATE TABLE module_translations (
    id INT PRIMARY KEY AUTO_INCREMENT,
    module_id INT NOT NULL,
    language_id INT NOT NULL,
    title VARCHAR(255),
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (module_id) REFERENCES modules(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE,
    UNIQUE KEY unique_module_language (module_id, language_id),
    INDEX idx_language (language_id),
    INDEX idx_module (module_id)
);

-- Insert default English language
INSERT INTO languages (code, name, native_name, is_default, is_rtl, enabled) 
VALUES ('en', 'English', 'English', TRUE, FALSE, TRUE);
