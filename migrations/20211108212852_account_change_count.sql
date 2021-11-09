-- Add migration script here
ALTER TABLE accounts
  ADD profile_image_change_count INTEGER NOT NULL DEFAULT 0;