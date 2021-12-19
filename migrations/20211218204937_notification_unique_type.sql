-- Add migration script here
CREATE UNIQUE INDEX unique_index_account_id_type_on_notification_inboxes ON notification_inboxes (account_id,_type);