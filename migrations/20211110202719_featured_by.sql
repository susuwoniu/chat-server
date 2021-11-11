-- Add migration script here
ALTER TABLE post_templates
  ALTER featured_by DROP NOT NULL;