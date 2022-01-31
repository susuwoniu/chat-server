-- Add migration script here
ALTER TABLE account_views 
 ADD COLUMN time_cursor bigint DEFAULT 0 NOT NULL;