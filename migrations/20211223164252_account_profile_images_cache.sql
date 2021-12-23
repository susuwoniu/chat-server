-- Add migration script here
ALTER TABLE accounts
  ADD profile_images json;
