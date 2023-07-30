-- Add down migration script here
-- Drop the Comments table
DROP TABLE IF EXISTS comments;

-- Drop the Users table
DROP TABLE IF EXISTS users;


DROP TYPE IF EXISTS Role; 