-- Add migration script here
ALTER TABLE reports 
  DROP COLUMN images,
  ADD COLUMN images json;