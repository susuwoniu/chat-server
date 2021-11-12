-- Add migration script here
ALTER TABLE posts
  RENAME skip_count TO skipped_count;
ALTER TABLE posts
  RENAME reply_count TO replied_count;
ALTER TABLE posts
  RENAME view_count TO viewed_count;