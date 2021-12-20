-- Add migration script here

ALTER TABLE notification_inboxes
  ADD total_count bigint DEFAULT 0 NOT NULL;