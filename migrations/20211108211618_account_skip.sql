-- Add migration script here
ALTER TABLE accounts
  ADD skip_optional_info BOOLEAN NOT NULL DEFAULT false;