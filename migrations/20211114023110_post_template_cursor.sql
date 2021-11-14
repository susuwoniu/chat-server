-- Add migration script here
ALTER TABLE post_templates
  ADD time_cursor bigint NOT NULL DEFAULT 0,
  ADD priority bigint;