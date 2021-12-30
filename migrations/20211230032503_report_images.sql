-- Add migration script here
ALTER TABLE reports 
  DROP COLUMN IF EXISTS images,
  ADD COLUMN images varchar(2048)[] NOT NULL DEFAULT array[]::varchar[];