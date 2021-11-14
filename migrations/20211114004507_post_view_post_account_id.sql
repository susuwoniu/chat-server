-- Add migration script here
ALTER TABLE post_view
  ALTER post_account_id SET NOT NULL;