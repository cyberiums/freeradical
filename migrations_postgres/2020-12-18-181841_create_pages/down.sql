-- Drop tables in reverse order due to foreign key constraints
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS modules CASCADE;
DROP TABLE IF EXISTS module_category CASCADE;
DROP TABLE IF EXISTS pages CASCADE;
