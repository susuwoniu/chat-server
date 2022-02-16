-- Add migration script here

ALTER TABLE accounts 
  ADD favorite_count bigint DEFAULT 0 NOT NULL;