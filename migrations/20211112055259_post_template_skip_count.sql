-- Add migration script here
ALTER TABLE post_templates
  RENAME skip_count TO skipped_count;
