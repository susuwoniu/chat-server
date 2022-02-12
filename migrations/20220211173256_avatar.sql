-- Add migration script here
ALTER TABLE accounts 
  DROP COLUMN avatar,
  ADD COLUMN avatar json;