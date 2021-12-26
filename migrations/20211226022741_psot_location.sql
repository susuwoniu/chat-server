-- Add migration script here
CREATE EXTENSION IF NOT EXISTS postgis;
ALTER TABLE posts
  ADD COLUMN geom geometry(point, 4326);
create index posts_location_index_geom on posts using gist(geom);
