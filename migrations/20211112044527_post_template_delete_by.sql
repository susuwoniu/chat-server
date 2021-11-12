-- Add migration script here
ALTER TABLE post_templates
  DROP deleted_at,
  ADD deleted_at timestamp without time zone,
  ADD deleted_by bigint;