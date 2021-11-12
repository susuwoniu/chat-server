-- Add migration script here
ALTER TABLE posts
  DROP time_cursor,
  ADD time_cursor bigint NOT NULL;