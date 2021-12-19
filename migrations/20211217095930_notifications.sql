-- Add migration script here
CREATE TABLE notifications (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    _type character varying(255) NOT NULL,
    content text DEFAULT ''::text NOT NULL,
    _action character varying(255) NOT NULL,
    from_account_id bigint,
    action_data json,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone
);
CREATE INDEX index_for_account_id_of_notifications ON notifications USING btree (account_id,deleted);
CREATE INDEX index_for_account_id_and_type_of_notifications ON notifications USING btree (account_id,_type,deleted);

CREATE TABLE notification_inboxes (
  id bigint NOT NULL PRIMARY KEY,
  created_at timestamp without time zone DEFAULT now() NOT NULL,
  updated_at timestamp without time zone NOT NULL,
  account_id bigint NOT NULL,
  unread_count bigint DEFAULT 0 NOT NULL,
  _type character varying(255) NOT NULL,
  last_notification_id bigint,
  last_notification_updated_at timestamp without time zone NOT NULL,
  last_notification_from bigint
);
CREATE INDEX index_for_account_id_of_notifications_inboxes ON notifications USING btree (account_id);
CREATE INDEX index_for_account_id_and_type_of_notifications_inboxes ON notifications USING btree (account_id,_type);