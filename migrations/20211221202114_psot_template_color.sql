-- Add migration script here
ALTER TABLE post_templates
  DROP COLUMN background_color;

ALTER TABLE posts
  DROP background_color,
  ADD background_color int NOT NULL,
  ADD color int NOT NULL;

