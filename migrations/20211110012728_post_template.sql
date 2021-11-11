-- Add migration script here
ALTER TABLE post_templates
  RENAME COLUMN verified TO featured;
ALTER TABLE post_templates
  ADD featured_at timestamp without time zone,
  ADD featured_by bigint NOT NULL,
  Add deleted boolean DEFAULT false NOT NULL,
  Add deleted_at boolean DEFAULT false NOT NULL,
  Add sensitive boolean DEFAULT false NOT NULL,
  Add ip inet;