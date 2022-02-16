-- Add migration script here
ALTER TABLE posts 
  ADD COLUMN favorite_count bigint DEFAULT 0 NOT NULL;