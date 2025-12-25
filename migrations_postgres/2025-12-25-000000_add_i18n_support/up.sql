-- Add multi-language support tables

CREATE TABLE languages (
    id INTEGERPRIMARY KEY SERIAL,
    code VARCHAR(10) UNIQUE NOT NULL ,
    name VARCHAR(100) NOT NULL ,
    native_name VARCHAR(100) ,
    is_default BOOLEAN DEFAULT FALSE ,
    is_rtl BOOLEAN DEFAULT FALSE ,
    enabled BOOLEAN DEFAULT TRUE ,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE page_translations (
    id INTEGERPRIMARY KEY SERIAL,
    page_id INTEGERNOT NULL,
    language_id INTEGERNOT NULL,
    page_title VARCHAR(255),
    page_content TEXT,
    page_url VARCHAR(255) ,
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
    id INTEGERPRIMARY KEY SERIAL,
    module_id INTEGERNOT NULL,
    language_id INTEGERNOT NULL,
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
INSERT INTEGER languages (code, name, native_name, is_default, is_rtl, enabled) 
VALUES ('en', 'English', 'English', TRUE, FALSE, TRUE);
