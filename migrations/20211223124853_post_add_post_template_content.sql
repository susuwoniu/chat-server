-- Add migration script here
ALTER TABLE post_templates
  ADD title char varying(255) NOT NULL;
ALTER TABLE posts
  ADD post_template_title char varying(255) NOT NULL;
