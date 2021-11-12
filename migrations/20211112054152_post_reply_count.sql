-- Add migration script here
ALTER TABLE posts
  ADD reply_count bigint NOT NULL DEFAULT 0;