-- Add migration script here
-- Add migration script here
ALTER TABLE posts
  DROP visibility;

DROP TYPE IF EXISTS visibility;

-- Create post svisibility
CREATE TYPE visibility AS ENUM ('public', 'unlisted','friend','private');



-- Create target_gender

ALTER TABLE posts
  ADD visibility visibility DEFAULT 'public' NOT NULL;