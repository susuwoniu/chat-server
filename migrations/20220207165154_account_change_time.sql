-- Add migration script here
ALTER TABLE accounts 
  ADD COLUMN bio_updated_at timestamp without time zone,
  ADD COLUMN name_updated_at timestamp without time zone,
  ADD COLUMN gender_updated_at timestamp without time zone,
  ADD COLUMN birthday_updated_at timestamp without time zone,
  ADD COLUMN phone_updated_at timestamp without time zone,
  DROP COLUMN skip_optional_info;
