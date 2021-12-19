-- Add migration script here
ALTER TABLE notification_inboxes
  ALTER last_notification_id SET NOT NULL;