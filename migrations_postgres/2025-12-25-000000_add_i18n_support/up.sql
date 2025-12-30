-- Add multi-language support tables

CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    native_name VARCHAR(100),
    is_default BOOLEAN DEFAULT FALSE,
    is_rtl BOOLEAN DEFAULT FALSE,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('languages');

CREATE TABLE page_translations (
    id SERIAL PRIMARY KEY,
    page_id VARCHAR(255) NOT NULL REFERENCES pages(uuid) ON DELETE CASCADE,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    page_title VARCHAR(255),
    page_content TEXT,
    page_url VARCHAR(255),
    meta_title VARCHAR(255),
    meta_description TEXT,
    og_title VARCHAR(255),
    og_description TEXT,
    twitter_title VARCHAR(255),
    twitter_description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_page_translation UNIQUE (page_id, language_id)
);

CREATE INDEX idx_pt_language ON page_translations (language_id);
CREATE INDEX idx_pt_page ON page_translations (page_id);

SELECT diesel_manage_updated_at('page_translations');

CREATE TABLE module_translations (
    id SERIAL PRIMARY KEY,
    module_id VARCHAR(255) NOT NULL REFERENCES modules(uuid) ON DELETE CASCADE,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    title VARCHAR(255),
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_module_translation UNIQUE (module_id, language_id)
);

CREATE INDEX idx_mt_language ON module_translations (language_id);
CREATE INDEX idx_mt_module ON module_translations (module_id);

SELECT diesel_manage_updated_at('module_translations');

-- Insert default English language
INSERT INTO languages (code, name, native_name, is_default, is_rtl, enabled) 
VALUES ('en', 'English', 'English', TRUE, FALSE, TRUE);
