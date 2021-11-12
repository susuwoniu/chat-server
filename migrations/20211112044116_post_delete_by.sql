-- Add migration script here
ALTER TABLE posts
  ADD deleted_by bigint;