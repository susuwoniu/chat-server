-- Add migration script here
ALTER TABLE posts
  DROP background_color,
  ADD background_color bigint NOT NULL,
  DROP color,
  ADD color bigint NOT NULL;

