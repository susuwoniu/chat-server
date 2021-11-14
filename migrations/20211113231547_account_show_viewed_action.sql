-- Add migration script here
ALTER TABLE accounts
  ADD show_viewed_action BOOLEAN NOT NULL DEFAULT true;