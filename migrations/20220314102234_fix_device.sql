-- Add migration script here
-- Add migration script here
ALTER TABLE public.devices 
  DROP COLUMN device_token ,
  ADD COLUMN device_token character varying NOT NULL;