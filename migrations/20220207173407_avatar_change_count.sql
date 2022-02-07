-- Add migration script here
ALTER TABLE accounts 
  ADD COLUMN avatar_change_count integer DEFAULT 0 NOT NULL;