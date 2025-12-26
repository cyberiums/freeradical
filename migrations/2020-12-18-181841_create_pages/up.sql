-- Enable UUID extension for PostgreSQL
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS pages (
    uuid varchar(255) PRIMARY KEY,
    page_name varchar(500) NOT NULL,
    page_url varchar(255) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO pages (page_name, uuid, page_url, page_title) 
VALUES ('index', uuid_generate_v4()::varchar, '/', 'Home')
ON CONFLICT (uuid) DO NOTHING;

CREATE TABLE IF NOT EXISTS module_category (
    uuid varchar(255) PRIMARY KEY,
    page_uuid varchar(255) NOT NULL,
    title varchar(255) NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE
);

INSERT INTO module_category (uuid, page_uuid, title) 
VALUES (uuid_generate_v4()::varchar, (SELECT uuid FROM pages LIMIT 1), 'colors')
ON CONFLICT (uuid) DO NOTHING;

CREATE TABLE IF NOT EXISTS modules (
    uuid varchar(255) PRIMARY KEY,
    page_uuid VARCHAR(255) NOT NULL,
    category_uuid VARCHAR(255),
    title varchar(255) NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE,
    FOREIGN KEY (category_uuid) REFERENCES module_category(uuid) ON DELETE CASCADE
);

INSERT INTO modules (uuid, title, page_uuid, content) 
VALUES (uuid_generate_v4()::varchar, 'title', (SELECT uuid FROM pages LIMIT 1), 'Welcome to Radical.')
ON  CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content) 
VALUES (uuid_generate_v4()::varchar, 'small', (SELECT uuid FROM pages LIMIT 1), 'A Rusty Wordpress Replacement')
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content) 
VALUES (uuid_generate_v4()::varchar, 'githublink', (SELECT uuid FROM pages LIMIT 1), 'https://github.com/Rust-CMS/radical')
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content) 
VALUES (uuid_generate_v4()::varchar, 'githublink_tooling', (SELECT uuid FROM pages LIMIT 1), 'https://github.com/Rust-CMS/tooling')
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content, category_uuid) 
VALUES (uuid_generate_v4()::varchar, 'color1', (SELECT uuid FROM pages LIMIT 1), 'red', (SELECT uuid FROM module_category LIMIT 1))
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content, category_uuid) 
VALUES (uuid_generate_v4()::varchar, 'color2', (SELECT uuid FROM pages LIMIT 1), 'blue', (SELECT uuid FROM module_category LIMIT 1))
ON CONFLICT (uuid) DO NOTHING;

INSERT INTO modules (uuid, title, page_uuid, content, category_uuid) 
VALUES (uuid_generate_v4()::varchar, 'color3', (SELECT uuid FROM pages LIMIT 1), 'green', (SELECT uuid from module_category LIMIT 1))
ON CONFLICT (uuid) DO NOTHING;

CREATE TABLE IF NOT EXISTS users (
    uuid varchar(255) PRIMARY KEY,
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    token varchar(511)
);

INSERT INTO users (uuid, username, password) 
VALUES (uuid_generate_v4()::varchar, 'root', '')
ON CONFLICT (username) DO NOTHING;