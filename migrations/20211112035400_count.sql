-- Add migration script here
ALTER TABLE accounts
  RENAME COLUMN post_templates_count TO post_template_count;


ALTER TABLE accounts
  RENAME COLUMN posts_count TO post_count;

ALTER TABLE accounts
  RENAME COLUMN likes_count TO like_count;