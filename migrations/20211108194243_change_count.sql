-- Add migration script here
ALTER TABLE accounts
  ADD name_change_count INTEGER NOT NULL DEFAULT 0,
  ADD bio_change_count INTEGER NOT NULL DEFAULT 0,
  ADD gender_change_count INTEGER NOT NULL DEFAULT 0,
  ADD birthday_change_count INTEGER NOT NULL DEFAULT 0,
  ADD phone_change_count INTEGER NOT NULL DEFAULT 0;