-- Add migration script here
ALTER TABLE accounts
  ADD post_templates_count bigint DEFAULT 0  NOT NULL;
