-- Add migration script here
ALTER TABLE account_view_records
  RENAME account_id TO viewed_by;
ALTER TABLE account_views
  RENAME account_id TO viewed_by;