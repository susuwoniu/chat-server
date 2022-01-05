-- Add migration script here
ALTER TABLE accounts
  ADD COLUMN last_post_created_at timestamp without time zone;