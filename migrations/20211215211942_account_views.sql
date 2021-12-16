-- Add migration script here
-- accountviews 表

CREATE TABLE account_view_records (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL
);
CREATE INDEX index_records_view_on_account_id ON account_view_records USING btree (account_id);
CREATE INDEX index_records_view_on_target_account_id ON account_view_records USING btree (target_account_id);


CREATE TABLE account_views (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL,
    view_count int NOT NULL DEFAULT 0
);
CREATE INDEX index_view_on_account_id ON account_views USING btree (account_id);
CREATE INDEX index_view_on_target_account_id ON account_views USING btree (target_account_id);
CREATE UNIQUE INDEX unique_index_account_views_on_account_id_and_viewed_by ON account_views (account_id,target_account_id);