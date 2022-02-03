-- Add migration script here
ALTER TABLE accounts 
 ADD COLUMN agree_community_rules_at timestamp without time zone;