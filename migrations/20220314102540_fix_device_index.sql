-- Add migration script here
-- DROP INDEX index_device_platform;

CREATE UNIQUE INDEX index_device_platform ON public.devices USING btree (client_platform, service_type,device_token,COALESCE(deleted_at, '0001-01-01T00:00:00Z'));