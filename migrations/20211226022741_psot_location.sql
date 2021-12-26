-- Add migration script here
create extension postgis;

ALTER TABLE posts
  ADD COLUMN geom geometry(point, 4326);
create index posts_location_index_geom on posts using gist(geom);
