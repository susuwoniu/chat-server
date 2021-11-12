-- Add migration script here
-- add skip table
CREATE TABLE post_skip (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    skipped_by bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);
CREATE UNIQUE INDEX unique_index_post_skip_on_post_id_and_skipped_by ON post_skip (post_id,skipped_by);
CREATE INDEX index_post_skip_on_skipped_by ON post_skip (skipped_by);
CREATE INDEX index_post_skip_on_post_account_id ON post_skip (post_account_id);


-- add post reply table
CREATE TABLE post_reply (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    replied_by bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);
CREATE UNIQUE INDEX unique_index_post_reply_on_post_id_and_replied_by ON post_reply (post_id,replied_by);
CREATE INDEX index_post_reply_on_replied_by ON post_reply USING btree (replied_by);
CREATE INDEX index_post_reply_on_post_account_id ON post_reply USING btree (post_account_id);