-- Add migration script here
ALTER TABLE notifications
  ADD is_primary boolean DEFAULT false NOT NULL;
ALTER TABLE notification_inboxes
  ADD is_primary boolean DEFAULT false NOT NULL;