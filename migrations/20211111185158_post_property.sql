-- Add migration script here
ALTER TABLE posts
  ADD approved boolean DEFAULT true NOT NULL,
  ADD approved_at timestamp without time zone DEFAULT NULL,
  ADD approved_by bigint DEFAULT NULL,
  ADD featured boolean DEFAULT false NOT NULL,
  ADD featured_at timestamp without time zone DEFAULT NULL,
  ADD featured_by bigint DEFAULT NULL;