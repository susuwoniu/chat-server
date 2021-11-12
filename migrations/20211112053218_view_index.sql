-- Add migration script here
ALTER TABLE post_view
  RENAME account_id TO viewed_by;
ALTER TABLE post_view
  ADD post_account_id bigint;
CREATE UNIQUE INDEX unique_index_post_view_on_post_id_and_viewed_by ON post_view (post_id,viewed_by);
