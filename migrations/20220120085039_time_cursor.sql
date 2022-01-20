-- Add migration script here
ALTER TABLE posts
  ADD COLUMN time_cursor_change_count integer DEFAULT 0 NOT NULL;