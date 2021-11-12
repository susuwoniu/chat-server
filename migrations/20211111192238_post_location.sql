-- Add migration script here
ALTER TABLE posts
  DROP target_gender;

DROP TYPE IF EXISTS target_gender;

-- Create post svisibility
CREATE TYPE visibility AS ENUM ('public', 'unlisted','friend','self');



-- Create target_gender

ALTER TABLE posts
  ADD target_gender gender DEFAULT NULL,
  ADD visibility visibility DEFAULT 'public' NOT NULL;