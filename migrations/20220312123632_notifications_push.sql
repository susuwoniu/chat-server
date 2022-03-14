-- Add migration script here
-- service_type: 0, jiguang
CREATE TABLE public.devices (
    id bigint NOT NULL,
    device_token character NOT NULL,
    service_type smallint DEFAULT 0 NOT NULL,
    account_id bigint,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    client_platform smallint DEFAULT 1 NOT NULL
);
CREATE UNIQUE INDEX index_device_platform ON public.devices USING btree (client_platform, service_type,device_token,COALESCE(deleted_at, '0001-01-01T00:00:00Z'));

CREATE  INDEX device_account_id ON public.devices USING btree (account_id);