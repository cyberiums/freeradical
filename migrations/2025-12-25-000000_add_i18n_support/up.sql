-- Add multi-language support tables

CREATE TABLE IF NOT EXISTS languages (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) UNIQUE NOT NULL, -- ISO 639-1 code (en, es, fr, de, ja),
    name VARCHAR(100) NOT NULL, -- English name,
    native_name VARCHAR(100), -- Native name (Español, Français),
    is_default BOOLEAN DEFAULT FALSE, -- Default language for site,
    is_rtl BOOLEAN DEFAULT FALSE, -- Right-to-left language,
    enabled BOOLEAN DEFAULT TRUE, -- Language is active,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS page_translations (
    id SERIAL PRIMARY KEY,
    page_id INT NOT NULL,
    language_id INT NOT NULL,
    page_title VARCHAR(255),
    page_content TEXT,
    page_url VARCHAR(255), -- Translated slug,
    meta_title VARCHAR(255),
    meta_description TEXT,
    og_title VARCHAR(255),
    og_description TEXT,
    twitter_title VARCHAR(255),
    twitter_description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE,
    UNIQUE (page_id, language_id)
);

CREATE INDEX IF NOT EXISTS idx_language ON page_translations(language_id);
CREATE INDEX IF NOT EXISTS idx_page ON page_translations(page_id);

CREATE TABLE IF NOT EXISTS module_translations (
    id SERIAL PRIMARY KEY,
    module_id INT NOT NULL,
    language_id INT NOT NULL,
    title VARCHAR(255),
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE,
    UNIQUE (module_id, language_id)
);

CREATE INDEX IF NOT EXISTS idx_language ON module_translations(language_id);
CREATE INDEX IF NOT EXISTS idx_module ON module_translations(module_id);

-- Insert default English language
INSERT INTO languages (code, name, native_name, is_default, is_rtl, enabled) 
VALUES ('en', 'English', 'English', TRUE, FALSE, TRUE);
