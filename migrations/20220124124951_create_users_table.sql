-- Add migration script here
-- Create users Table
CREATE TABLE users(
   id VARCHAR(48) NOT NULL UNIQUE,
   PRIMARY KEY (id),
   name VARCHAR(64) NOT NULL UNIQUE,
   email TEXT NOT NULL UNIQUE,
   created_at timestamptz NOT NULL
);