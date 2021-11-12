-- Add migration script here
CREATE INDEX index_post_view_on_viewed_by ON post_view (viewed_by);
CREATE INDEX index_post_view_on_post_account_id ON post_view (post_account_id);

