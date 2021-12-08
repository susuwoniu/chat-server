-- Add migration script here
ALTER TABLE account_images
  ADD height FLOAT8 NOT NULL,
  ADD width FLOAT8 NOT NULL,
  ADD size bigint NOT NULL,
  ADD mime_type varchar(255) NOT NULL;