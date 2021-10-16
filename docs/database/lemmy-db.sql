--
-- PostgreSQL database dump
--

-- Dumped from database version 13.4
-- Dumped by pg_dump version 13.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: utils; Type: SCHEMA; Schema: -; Owner: owenyoung
--

CREATE SCHEMA utils;


ALTER SCHEMA utils OWNER TO owenyoung;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: comment_aggregates_comment(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.comment_aggregates_comment() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into comment_aggregates (comment_id, published) values (NEW.id, NEW.published);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from comment_aggregates where comment_id = OLD.id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.comment_aggregates_comment() OWNER TO owenyoung;

--
-- Name: comment_aggregates_score(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.comment_aggregates_score() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update comment_aggregates ca
    set score = score + NEW.score,
    upvotes = case when NEW.score = 1 then upvotes + 1 else upvotes end,
    downvotes = case when NEW.score = -1 then downvotes + 1 else downvotes end
    where ca.comment_id = NEW.comment_id;

  ELSIF (TG_OP = 'DELETE') THEN
    -- Join to comment because that comment may not exist anymore
    update comment_aggregates ca
    set score = score - OLD.score,
    upvotes = case when OLD.score = 1 then upvotes - 1 else upvotes end,
    downvotes = case when OLD.score = -1 then downvotes - 1 else downvotes end
    from comment c
    where ca.comment_id = c.id
    and ca.comment_id = OLD.comment_id;

  END IF;
  return null;
end $$;


ALTER FUNCTION public.comment_aggregates_score() OWNER TO owenyoung;

--
-- Name: community_aggregates_activity(text); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.community_aggregates_activity(i text) RETURNS TABLE(count_ bigint, community_id_ integer)
    LANGUAGE plpgsql
    AS $$
begin
  return query 
  select count(*), community_id
  from (
    select c.creator_id, p.community_id from comment c
    inner join post p on c.post_id = p.id
    inner join person pe on c.creator_id = pe.id
    where c.published > ('now'::timestamp - i::interval)
    and pe.bot_account = false
    union
    select p.creator_id, p.community_id from post p
    inner join person pe on p.creator_id = pe.id
    where p.published > ('now'::timestamp - i::interval)  
    and pe.bot_account = false
  ) a
  group by community_id;
end;
$$;


ALTER FUNCTION public.community_aggregates_activity(i text) OWNER TO owenyoung;

--
-- Name: community_aggregates_comment_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.community_aggregates_comment_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update community_aggregates ca
    set comments = comments + 1 from comment c, post p
    where p.id = c.post_id 
    and p.id = NEW.post_id 
    and ca.community_id = p.community_id;
  ELSIF (TG_OP = 'DELETE') THEN
    update community_aggregates ca
    set comments = comments - 1 from comment c, post p
    where p.id = c.post_id 
    and p.id = OLD.post_id 
    and ca.community_id = p.community_id;

  END IF;
  return null;
end $$;


ALTER FUNCTION public.community_aggregates_comment_count() OWNER TO owenyoung;

--
-- Name: community_aggregates_community(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.community_aggregates_community() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into community_aggregates (community_id, published) values (NEW.id, NEW.published);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from community_aggregates where community_id = OLD.id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.community_aggregates_community() OWNER TO owenyoung;

--
-- Name: community_aggregates_post_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.community_aggregates_post_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update community_aggregates 
    set posts = posts + 1 where community_id = NEW.community_id;
  ELSIF (TG_OP = 'DELETE') THEN
    update community_aggregates 
    set posts = posts - 1 where community_id = OLD.community_id;

    -- Update the counts if the post got deleted
    update community_aggregates ca
    set posts = coalesce(cd.posts, 0),
    comments = coalesce(cd.comments, 0)
    from ( 
      select 
      c.id,
      count(distinct p.id) as posts,
      count(distinct ct.id) as comments
      from community c
      left join post p on c.id = p.community_id
      left join comment ct on p.id = ct.post_id
      group by c.id
    ) cd 
    where ca.community_id = OLD.community_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.community_aggregates_post_count() OWNER TO owenyoung;

--
-- Name: community_aggregates_subscriber_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.community_aggregates_subscriber_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update community_aggregates 
    set subscribers = subscribers + 1 where community_id = NEW.community_id;
  ELSIF (TG_OP = 'DELETE') THEN
    update community_aggregates 
    set subscribers = subscribers - 1 where community_id = OLD.community_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.community_aggregates_subscriber_count() OWNER TO owenyoung;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO owenyoung;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO owenyoung;

--
-- Name: generate_unique_changeme(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.generate_unique_changeme() RETURNS text
    LANGUAGE sql
    AS $$
  select 'http://changeme_' || string_agg (substr('abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz0123456789', ceil (random() * 62)::integer, 1), '')
  from generate_series(1, 20)
$$;


ALTER FUNCTION public.generate_unique_changeme() OWNER TO owenyoung;

--
-- Name: hot_rank(numeric, timestamp without time zone); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.hot_rank(score numeric, published timestamp without time zone) RETURNS integer
    LANGUAGE plpgsql IMMUTABLE
    AS $$
begin
  -- hours_diff:=EXTRACT(EPOCH FROM (timezone('utc',now()) - published))/3600
  return floor(10000*log(greatest(1,score+3)) / power(((EXTRACT(EPOCH FROM (timezone('utc',now()) - published))/3600) + 2), 1.8))::integer;
end; $$;


ALTER FUNCTION public.hot_rank(score numeric, published timestamp without time zone) OWNER TO owenyoung;

--
-- Name: person_aggregates_comment_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.person_aggregates_comment_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update person_aggregates 
    set comment_count = comment_count + 1 where person_id = NEW.creator_id;
  ELSIF (TG_OP = 'DELETE') THEN
    update person_aggregates 
    set comment_count = comment_count - 1 where person_id = OLD.creator_id;

    -- If the comment gets deleted, the score calculation trigger won't fire, 
    -- so you need to re-calculate
    update person_aggregates ua
    set comment_score = cd.score
    from (
      select u.id,
      coalesce(0, sum(cl.score)) as score
      -- User join because comments could be empty
      from person u 
      left join comment c on u.id = c.creator_id
      left join comment_like cl on c.id = cl.comment_id
      group by u.id
    ) cd 
    where ua.person_id = OLD.creator_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.person_aggregates_comment_count() OWNER TO owenyoung;

--
-- Name: person_aggregates_comment_score(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.person_aggregates_comment_score() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    -- Need to get the post creator, not the voter
    update person_aggregates ua
    set comment_score = comment_score + NEW.score
    from comment c
    where ua.person_id = c.creator_id and c.id = NEW.comment_id;
  ELSIF (TG_OP = 'DELETE') THEN
    update person_aggregates ua
    set comment_score = comment_score - OLD.score
    from comment c
    where ua.person_id = c.creator_id and c.id = OLD.comment_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.person_aggregates_comment_score() OWNER TO owenyoung;

--
-- Name: person_aggregates_person(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.person_aggregates_person() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into person_aggregates (person_id) values (NEW.id);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from person_aggregates where person_id = OLD.id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.person_aggregates_person() OWNER TO owenyoung;

--
-- Name: person_aggregates_post_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.person_aggregates_post_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update person_aggregates 
    set post_count = post_count + 1 where person_id = NEW.creator_id;

  ELSIF (TG_OP = 'DELETE') THEN
    update person_aggregates 
    set post_count = post_count - 1 where person_id = OLD.creator_id;

    -- If the post gets deleted, the score calculation trigger won't fire, 
    -- so you need to re-calculate
    update person_aggregates ua
    set post_score = pd.score
    from (
      select u.id,
      coalesce(0, sum(pl.score)) as score
      -- User join because posts could be empty
      from person u 
      left join post p on u.id = p.creator_id
      left join post_like pl on p.id = pl.post_id
      group by u.id
    ) pd 
    where ua.person_id = OLD.creator_id;

  END IF;
  return null;
end $$;


ALTER FUNCTION public.person_aggregates_post_count() OWNER TO owenyoung;

--
-- Name: person_aggregates_post_score(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.person_aggregates_post_score() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    -- Need to get the post creator, not the voter
    update person_aggregates ua
    set post_score = post_score + NEW.score
    from post p
    where ua.person_id = p.creator_id and p.id = NEW.post_id;
    
  ELSIF (TG_OP = 'DELETE') THEN
    update person_aggregates ua
    set post_score = post_score - OLD.score
    from post p
    where ua.person_id = p.creator_id and p.id = OLD.post_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.person_aggregates_post_score() OWNER TO owenyoung;

--
-- Name: post_aggregates_comment_count(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.post_aggregates_comment_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update post_aggregates pa
    set comments = comments + 1,
    newest_comment_time = NEW.published
    where pa.post_id = NEW.post_id;

    -- A 2 day necro-bump limit
    update post_aggregates pa
    set newest_comment_time_necro = NEW.published
    from post p
    where pa.post_id = p.id
    and pa.post_id = NEW.post_id
    -- Fix issue with being able to necro-bump your own post
    and NEW.creator_id != p.creator_id
    and pa.published > ('now'::timestamp - '2 days'::interval);

  ELSIF (TG_OP = 'DELETE') THEN
    -- Join to post because that post may not exist anymore
    update post_aggregates pa
    set comments = comments - 1
    from post p
    where pa.post_id = p.id
    and pa.post_id = OLD.post_id;
  ELSIF (TG_OP = 'UPDATE') THEN
    -- Join to post because that post may not exist anymore
    update post_aggregates pa
    set comments = comments - 1
    from post p
    where pa.post_id = p.id
    and pa.post_id = OLD.post_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.post_aggregates_comment_count() OWNER TO owenyoung;

--
-- Name: post_aggregates_comment_deleted(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.post_aggregates_comment_deleted() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF NEW.deleted = TRUE THEN
    update post_aggregates pa
    set comments = comments - 1
    where pa.post_id = NEW.post_id;
  ELSE 
    update post_aggregates pa
    set comments = comments + 1
    where pa.post_id = NEW.post_id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.post_aggregates_comment_deleted() OWNER TO owenyoung;

--
-- Name: post_aggregates_post(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.post_aggregates_post() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into post_aggregates (post_id, published, newest_comment_time, newest_comment_time_necro) values (NEW.id, NEW.published, NEW.published, NEW.published);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from post_aggregates where post_id = OLD.id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.post_aggregates_post() OWNER TO owenyoung;

--
-- Name: post_aggregates_score(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.post_aggregates_score() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    update post_aggregates pa
    set score = score + NEW.score,
    upvotes = case when NEW.score = 1 then upvotes + 1 else upvotes end,
    downvotes = case when NEW.score = -1 then downvotes + 1 else downvotes end
    where pa.post_id = NEW.post_id;

  ELSIF (TG_OP = 'DELETE') THEN
    -- Join to post because that post may not exist anymore
    update post_aggregates pa
    set score = score - OLD.score,
    upvotes = case when OLD.score = 1 then upvotes - 1 else upvotes end,
    downvotes = case when OLD.score = -1 then downvotes - 1 else downvotes end
    from post p
    where pa.post_id = p.id
    and pa.post_id = OLD.post_id;

  END IF;
  return null;
end $$;


ALTER FUNCTION public.post_aggregates_score() OWNER TO owenyoung;

--
-- Name: post_aggregates_stickied(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.post_aggregates_stickied() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update post_aggregates pa
  set stickied = NEW.stickied
  where pa.post_id = NEW.id;

  return null;
end $$;


ALTER FUNCTION public.post_aggregates_stickied() OWNER TO owenyoung;

--
-- Name: site_aggregates_activity(text); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_activity(i text) RETURNS integer
    LANGUAGE plpgsql
    AS $$
declare
   count_ integer;
begin
  select count(*)
  into count_
  from (
    select c.creator_id from comment c
    inner join person u on c.creator_id = u.id
    inner join person pe on c.creator_id = pe.id
    where c.published > ('now'::timestamp - i::interval) 
    and u.local = true
    and pe.bot_account = false
    union
    select p.creator_id from post p
    inner join person u on p.creator_id = u.id
    inner join person pe on p.creator_id = pe.id
    where p.published > ('now'::timestamp - i::interval)
    and u.local = true
    and pe.bot_account = false
  ) a;
  return count_;
end;
$$;


ALTER FUNCTION public.site_aggregates_activity(i text) OWNER TO owenyoung;

--
-- Name: site_aggregates_comment_delete(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_comment_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates sa
  set comments = comments - 1
  from site s
  where sa.site_id = s.id;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_comment_delete() OWNER TO owenyoung;

--
-- Name: site_aggregates_comment_insert(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_comment_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates 
  set comments = comments + 1;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_comment_insert() OWNER TO owenyoung;

--
-- Name: site_aggregates_community_delete(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_community_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates sa
  set communities = communities - 1
  from site s
  where sa.site_id = s.id;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_community_delete() OWNER TO owenyoung;

--
-- Name: site_aggregates_community_insert(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_community_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates 
  set communities = communities + 1;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_community_insert() OWNER TO owenyoung;

--
-- Name: site_aggregates_person_delete(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_person_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  -- Join to site since the creator might not be there anymore
  update site_aggregates sa
  set users = users - 1
  from site s
  where sa.site_id = s.id;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_person_delete() OWNER TO owenyoung;

--
-- Name: site_aggregates_person_insert(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_person_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates 
  set users = users + 1;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_person_insert() OWNER TO owenyoung;

--
-- Name: site_aggregates_post_delete(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_post_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates sa
  set posts = posts - 1
  from site s
  where sa.site_id = s.id;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_post_delete() OWNER TO owenyoung;

--
-- Name: site_aggregates_post_insert(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_post_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  update site_aggregates 
  set posts = posts + 1;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_post_insert() OWNER TO owenyoung;

--
-- Name: site_aggregates_site(); Type: FUNCTION; Schema: public; Owner: owenyoung
--

CREATE FUNCTION public.site_aggregates_site() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into site_aggregates (site_id) values (NEW.id);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from site_aggregates where site_id = OLD.id;
  END IF;
  return null;
end $$;


ALTER FUNCTION public.site_aggregates_site() OWNER TO owenyoung;

--
-- Name: restore_views(character varying, character varying); Type: FUNCTION; Schema: utils; Owner: owenyoung
--

CREATE FUNCTION utils.restore_views(p_view_schema character varying, p_view_name character varying) RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  v_curr record;
begin
for v_curr in 
(
  select ddl_to_run, id 
  from utils.deps_saved_ddl
  where view_schema = p_view_schema and view_name = p_view_name
  order by id desc
) loop
begin
  execute v_curr.ddl_to_run;
  delete from utils.deps_saved_ddl where id = v_curr.id;
  EXCEPTION WHEN OTHERS THEN
      -- keep looping, but please check for errors or remove left overs to handle manually
	  end;
end loop;
end;
$$;


ALTER FUNCTION utils.restore_views(p_view_schema character varying, p_view_name character varying) OWNER TO owenyoung;

--
-- Name: save_and_drop_views(name, name); Type: FUNCTION; Schema: utils; Owner: owenyoung
--

CREATE FUNCTION utils.save_and_drop_views(p_view_schema name, p_view_name name) RETURNS void
    LANGUAGE plpgsql
    AS $$

declare
  v_curr record;
begin
for v_curr in 
(
  select obj_schema, obj_name, obj_type from
  (
  with recursive recursive_deps(obj_schema, obj_name, obj_type, depth) as 
  (
    select p_view_schema::name, p_view_name, null::varchar, 0
    union
    select dep_schema::varchar, dep_name::varchar, dep_type::varchar, recursive_deps.depth + 1 from 
    (
      select ref_nsp.nspname ref_schema, ref_cl.relname ref_name, 
      rwr_cl.relkind dep_type,
      rwr_nsp.nspname dep_schema,
      rwr_cl.relname dep_name
      from pg_depend dep
      join pg_class ref_cl on dep.refobjid = ref_cl.oid
      join pg_namespace ref_nsp on ref_cl.relnamespace = ref_nsp.oid
      join pg_rewrite rwr on dep.objid = rwr.oid
      join pg_class rwr_cl on rwr.ev_class = rwr_cl.oid
      join pg_namespace rwr_nsp on rwr_cl.relnamespace = rwr_nsp.oid
      where dep.deptype = 'n'
      and dep.classid = 'pg_rewrite'::regclass
    ) deps
    join recursive_deps on deps.ref_schema = recursive_deps.obj_schema and deps.ref_name = recursive_deps.obj_name
    where (deps.ref_schema != deps.dep_schema or deps.ref_name != deps.dep_name)
  )
  select obj_schema, obj_name, obj_type, depth
  from recursive_deps 
  where depth > 0
  ) t
  group by obj_schema, obj_name, obj_type
  order by max(depth) desc
) loop
  if v_curr.obj_type = 'v' then
    insert into utils.deps_saved_ddl(view_schema, view_name, ddl_to_run)
    select p_view_schema, p_view_name, 'CREATE VIEW ' || v_curr.obj_schema || '.' || v_curr.obj_name || ' AS ' || view_definition
    from information_schema.views
    where table_schema = v_curr.obj_schema and table_name = v_curr.obj_name;

    execute 'DROP VIEW' || ' ' || v_curr.obj_schema || '.' || v_curr.obj_name;
  end if;
end loop;
end;
$$;


ALTER FUNCTION utils.save_and_drop_views(p_view_schema name, p_view_name name) OWNER TO owenyoung;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO owenyoung;

--
-- Name: activity; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.activity (
    id integer NOT NULL,
    data jsonb NOT NULL,
    local boolean DEFAULT true NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    ap_id text,
    sensitive boolean DEFAULT true
);


ALTER TABLE public.activity OWNER TO owenyoung;

--
-- Name: activity_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.activity_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.activity_id_seq OWNER TO owenyoung;

--
-- Name: activity_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.activity_id_seq OWNED BY public.activity.id;


--
-- Name: comment; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.comment (
    id integer NOT NULL,
    creator_id integer NOT NULL,
    post_id integer NOT NULL,
    parent_id integer,
    content text NOT NULL,
    removed boolean DEFAULT false NOT NULL,
    read boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    deleted boolean DEFAULT false NOT NULL,
    ap_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    local boolean DEFAULT true NOT NULL
);


ALTER TABLE public.comment OWNER TO owenyoung;

--
-- Name: comment_aggregates; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.comment_aggregates (
    id integer NOT NULL,
    comment_id integer NOT NULL,
    score bigint DEFAULT 0 NOT NULL,
    upvotes bigint DEFAULT 0 NOT NULL,
    downvotes bigint DEFAULT 0 NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.comment_aggregates OWNER TO owenyoung;

--
-- Name: comment_aggregates_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.comment_aggregates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comment_aggregates_id_seq OWNER TO owenyoung;

--
-- Name: comment_aggregates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.comment_aggregates_id_seq OWNED BY public.comment_aggregates.id;


--
-- Name: comment_alias_1; Type: VIEW; Schema: public; Owner: owenyoung
--

CREATE VIEW public.comment_alias_1 AS
 SELECT comment.id,
    comment.creator_id,
    comment.post_id,
    comment.parent_id,
    comment.content,
    comment.removed,
    comment.read,
    comment.published,
    comment.updated,
    comment.deleted,
    comment.ap_id,
    comment.local
   FROM public.comment;


ALTER TABLE public.comment_alias_1 OWNER TO owenyoung;

--
-- Name: comment_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.comment_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comment_id_seq OWNER TO owenyoung;

--
-- Name: comment_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.comment_id_seq OWNED BY public.comment.id;


--
-- Name: comment_like; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.comment_like (
    id integer NOT NULL,
    person_id integer NOT NULL,
    comment_id integer NOT NULL,
    post_id integer NOT NULL,
    score smallint NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.comment_like OWNER TO owenyoung;

--
-- Name: comment_like_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.comment_like_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comment_like_id_seq OWNER TO owenyoung;

--
-- Name: comment_like_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.comment_like_id_seq OWNED BY public.comment_like.id;


--
-- Name: comment_report; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.comment_report (
    id integer NOT NULL,
    creator_id integer NOT NULL,
    comment_id integer NOT NULL,
    original_comment_text text NOT NULL,
    reason text NOT NULL,
    resolved boolean DEFAULT false NOT NULL,
    resolver_id integer,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone
);


ALTER TABLE public.comment_report OWNER TO owenyoung;

--
-- Name: comment_report_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.comment_report_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comment_report_id_seq OWNER TO owenyoung;

--
-- Name: comment_report_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.comment_report_id_seq OWNED BY public.comment_report.id;


--
-- Name: comment_saved; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.comment_saved (
    id integer NOT NULL,
    comment_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.comment_saved OWNER TO owenyoung;

--
-- Name: comment_saved_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.comment_saved_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comment_saved_id_seq OWNER TO owenyoung;

--
-- Name: comment_saved_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.comment_saved_id_seq OWNED BY public.comment_saved.id;


--
-- Name: community; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    removed boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    deleted boolean DEFAULT false NOT NULL,
    nsfw boolean DEFAULT false NOT NULL,
    actor_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    local boolean DEFAULT true NOT NULL,
    private_key text,
    public_key text,
    last_refreshed_at timestamp without time zone DEFAULT now() NOT NULL,
    icon text,
    banner text,
    followers_url character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    inbox_url character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    shared_inbox_url character varying(255)
);


ALTER TABLE public.community OWNER TO owenyoung;

--
-- Name: community_aggregates; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community_aggregates (
    id integer NOT NULL,
    community_id integer NOT NULL,
    subscribers bigint DEFAULT 0 NOT NULL,
    posts bigint DEFAULT 0 NOT NULL,
    comments bigint DEFAULT 0 NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    users_active_day bigint DEFAULT 0 NOT NULL,
    users_active_week bigint DEFAULT 0 NOT NULL,
    users_active_month bigint DEFAULT 0 NOT NULL,
    users_active_half_year bigint DEFAULT 0 NOT NULL
);


ALTER TABLE public.community_aggregates OWNER TO owenyoung;

--
-- Name: community_aggregates_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_aggregates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_aggregates_id_seq OWNER TO owenyoung;

--
-- Name: community_aggregates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_aggregates_id_seq OWNED BY public.community_aggregates.id;


--
-- Name: community_block; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community_block (
    id integer NOT NULL,
    person_id integer NOT NULL,
    community_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.community_block OWNER TO owenyoung;

--
-- Name: community_block_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_block_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_block_id_seq OWNER TO owenyoung;

--
-- Name: community_block_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_block_id_seq OWNED BY public.community_block.id;


--
-- Name: community_follower; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community_follower (
    id integer NOT NULL,
    community_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    pending boolean DEFAULT false
);


ALTER TABLE public.community_follower OWNER TO owenyoung;

--
-- Name: community_follower_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_follower_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_follower_id_seq OWNER TO owenyoung;

--
-- Name: community_follower_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_follower_id_seq OWNED BY public.community_follower.id;


--
-- Name: community_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_id_seq OWNER TO owenyoung;

--
-- Name: community_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_id_seq OWNED BY public.community.id;


--
-- Name: community_moderator; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community_moderator (
    id integer NOT NULL,
    community_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.community_moderator OWNER TO owenyoung;

--
-- Name: community_moderator_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_moderator_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_moderator_id_seq OWNER TO owenyoung;

--
-- Name: community_moderator_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_moderator_id_seq OWNED BY public.community_moderator.id;


--
-- Name: community_person_ban; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.community_person_ban (
    id integer NOT NULL,
    community_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.community_person_ban OWNER TO owenyoung;

--
-- Name: community_person_ban_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.community_person_ban_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.community_person_ban_id_seq OWNER TO owenyoung;

--
-- Name: community_person_ban_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.community_person_ban_id_seq OWNED BY public.community_person_ban.id;


--
-- Name: local_user; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.local_user (
    id integer NOT NULL,
    person_id integer NOT NULL,
    password_encrypted text NOT NULL,
    email text,
    show_nsfw boolean DEFAULT false NOT NULL,
    theme character varying(20) DEFAULT 'darkly'::character varying NOT NULL,
    default_sort_type smallint DEFAULT 0 NOT NULL,
    default_listing_type smallint DEFAULT 1 NOT NULL,
    lang character varying(20) DEFAULT 'browser'::character varying NOT NULL,
    show_avatars boolean DEFAULT true NOT NULL,
    send_notifications_to_email boolean DEFAULT false NOT NULL,
    validator_time timestamp without time zone DEFAULT now() NOT NULL,
    show_scores boolean DEFAULT true NOT NULL,
    show_bot_accounts boolean DEFAULT true NOT NULL,
    show_read_posts boolean DEFAULT true NOT NULL,
    show_new_post_notifs boolean DEFAULT false NOT NULL
);


ALTER TABLE public.local_user OWNER TO owenyoung;

--
-- Name: local_user_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.local_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.local_user_id_seq OWNER TO owenyoung;

--
-- Name: local_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.local_user_id_seq OWNED BY public.local_user.id;


--
-- Name: mod_add; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_add (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    other_person_id integer NOT NULL,
    removed boolean DEFAULT false,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_add OWNER TO owenyoung;

--
-- Name: mod_add_community; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_add_community (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    other_person_id integer NOT NULL,
    community_id integer NOT NULL,
    removed boolean DEFAULT false,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_add_community OWNER TO owenyoung;

--
-- Name: mod_add_community_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_add_community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_add_community_id_seq OWNER TO owenyoung;

--
-- Name: mod_add_community_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_add_community_id_seq OWNED BY public.mod_add_community.id;


--
-- Name: mod_add_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_add_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_add_id_seq OWNER TO owenyoung;

--
-- Name: mod_add_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_add_id_seq OWNED BY public.mod_add.id;


--
-- Name: mod_ban; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_ban (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    other_person_id integer NOT NULL,
    reason text,
    banned boolean DEFAULT true,
    expires timestamp without time zone,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_ban OWNER TO owenyoung;

--
-- Name: mod_ban_from_community; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_ban_from_community (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    other_person_id integer NOT NULL,
    community_id integer NOT NULL,
    reason text,
    banned boolean DEFAULT true,
    expires timestamp without time zone,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_ban_from_community OWNER TO owenyoung;

--
-- Name: mod_ban_from_community_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_ban_from_community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_ban_from_community_id_seq OWNER TO owenyoung;

--
-- Name: mod_ban_from_community_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_ban_from_community_id_seq OWNED BY public.mod_ban_from_community.id;


--
-- Name: mod_ban_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_ban_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_ban_id_seq OWNER TO owenyoung;

--
-- Name: mod_ban_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_ban_id_seq OWNED BY public.mod_ban.id;


--
-- Name: mod_lock_post; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_lock_post (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    post_id integer NOT NULL,
    locked boolean DEFAULT true,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_lock_post OWNER TO owenyoung;

--
-- Name: mod_lock_post_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_lock_post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_lock_post_id_seq OWNER TO owenyoung;

--
-- Name: mod_lock_post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_lock_post_id_seq OWNED BY public.mod_lock_post.id;


--
-- Name: mod_remove_comment; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_remove_comment (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    comment_id integer NOT NULL,
    reason text,
    removed boolean DEFAULT true,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_remove_comment OWNER TO owenyoung;

--
-- Name: mod_remove_comment_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_remove_comment_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_remove_comment_id_seq OWNER TO owenyoung;

--
-- Name: mod_remove_comment_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_remove_comment_id_seq OWNED BY public.mod_remove_comment.id;


--
-- Name: mod_remove_community; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_remove_community (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    community_id integer NOT NULL,
    reason text,
    removed boolean DEFAULT true,
    expires timestamp without time zone,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_remove_community OWNER TO owenyoung;

--
-- Name: mod_remove_community_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_remove_community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_remove_community_id_seq OWNER TO owenyoung;

--
-- Name: mod_remove_community_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_remove_community_id_seq OWNED BY public.mod_remove_community.id;


--
-- Name: mod_remove_post; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_remove_post (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    post_id integer NOT NULL,
    reason text,
    removed boolean DEFAULT true,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_remove_post OWNER TO owenyoung;

--
-- Name: mod_remove_post_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_remove_post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_remove_post_id_seq OWNER TO owenyoung;

--
-- Name: mod_remove_post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_remove_post_id_seq OWNED BY public.mod_remove_post.id;


--
-- Name: mod_sticky_post; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_sticky_post (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    post_id integer NOT NULL,
    stickied boolean DEFAULT true,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_sticky_post OWNER TO owenyoung;

--
-- Name: mod_sticky_post_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_sticky_post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_sticky_post_id_seq OWNER TO owenyoung;

--
-- Name: mod_sticky_post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_sticky_post_id_seq OWNED BY public.mod_sticky_post.id;


--
-- Name: mod_transfer_community; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.mod_transfer_community (
    id integer NOT NULL,
    mod_person_id integer NOT NULL,
    other_person_id integer NOT NULL,
    community_id integer NOT NULL,
    removed boolean DEFAULT false,
    when_ timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mod_transfer_community OWNER TO owenyoung;

--
-- Name: mod_transfer_community_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.mod_transfer_community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mod_transfer_community_id_seq OWNER TO owenyoung;

--
-- Name: mod_transfer_community_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.mod_transfer_community_id_seq OWNED BY public.mod_transfer_community.id;


--
-- Name: password_reset_request; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.password_reset_request (
    id integer NOT NULL,
    token_encrypted text NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    local_user_id integer NOT NULL
);


ALTER TABLE public.password_reset_request OWNER TO owenyoung;

--
-- Name: password_reset_request_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.password_reset_request_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.password_reset_request_id_seq OWNER TO owenyoung;

--
-- Name: password_reset_request_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.password_reset_request_id_seq OWNED BY public.password_reset_request.id;


--
-- Name: person; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.person (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    display_name character varying(255),
    avatar text,
    banned boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    actor_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    bio text,
    local boolean DEFAULT true NOT NULL,
    private_key text,
    public_key text,
    last_refreshed_at timestamp without time zone DEFAULT now() NOT NULL,
    banner text,
    deleted boolean DEFAULT false NOT NULL,
    inbox_url character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    shared_inbox_url character varying(255),
    matrix_user_id text,
    admin boolean DEFAULT false NOT NULL,
    bot_account boolean DEFAULT false NOT NULL
);


ALTER TABLE public.person OWNER TO owenyoung;

--
-- Name: person_aggregates; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.person_aggregates (
    id integer NOT NULL,
    person_id integer NOT NULL,
    post_count bigint DEFAULT 0 NOT NULL,
    post_score bigint DEFAULT 0 NOT NULL,
    comment_count bigint DEFAULT 0 NOT NULL,
    comment_score bigint DEFAULT 0 NOT NULL
);


ALTER TABLE public.person_aggregates OWNER TO owenyoung;

--
-- Name: person_aggregates_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.person_aggregates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.person_aggregates_id_seq OWNER TO owenyoung;

--
-- Name: person_aggregates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.person_aggregates_id_seq OWNED BY public.person_aggregates.id;


--
-- Name: person_alias_1; Type: VIEW; Schema: public; Owner: owenyoung
--

CREATE VIEW public.person_alias_1 AS
 SELECT person.id,
    person.name,
    person.display_name,
    person.avatar,
    person.banned,
    person.published,
    person.updated,
    person.actor_id,
    person.bio,
    person.local,
    person.private_key,
    person.public_key,
    person.last_refreshed_at,
    person.banner,
    person.deleted,
    person.inbox_url,
    person.shared_inbox_url,
    person.matrix_user_id,
    person.admin,
    person.bot_account
   FROM public.person;


ALTER TABLE public.person_alias_1 OWNER TO owenyoung;

--
-- Name: person_alias_2; Type: VIEW; Schema: public; Owner: owenyoung
--

CREATE VIEW public.person_alias_2 AS
 SELECT person.id,
    person.name,
    person.display_name,
    person.avatar,
    person.banned,
    person.published,
    person.updated,
    person.actor_id,
    person.bio,
    person.local,
    person.private_key,
    person.public_key,
    person.last_refreshed_at,
    person.banner,
    person.deleted,
    person.inbox_url,
    person.shared_inbox_url,
    person.matrix_user_id,
    person.admin,
    person.bot_account
   FROM public.person;


ALTER TABLE public.person_alias_2 OWNER TO owenyoung;

--
-- Name: person_ban; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.person_ban (
    id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.person_ban OWNER TO owenyoung;

--
-- Name: person_ban_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.person_ban_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.person_ban_id_seq OWNER TO owenyoung;

--
-- Name: person_ban_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.person_ban_id_seq OWNED BY public.person_ban.id;


--
-- Name: person_block; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.person_block (
    id integer NOT NULL,
    person_id integer NOT NULL,
    target_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.person_block OWNER TO owenyoung;

--
-- Name: person_block_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.person_block_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.person_block_id_seq OWNER TO owenyoung;

--
-- Name: person_block_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.person_block_id_seq OWNED BY public.person_block.id;


--
-- Name: person_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.person_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.person_id_seq OWNER TO owenyoung;

--
-- Name: person_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.person_id_seq OWNED BY public.person.id;


--
-- Name: person_mention; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.person_mention (
    id integer NOT NULL,
    recipient_id integer NOT NULL,
    comment_id integer NOT NULL,
    read boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.person_mention OWNER TO owenyoung;

--
-- Name: person_mention_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.person_mention_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.person_mention_id_seq OWNER TO owenyoung;

--
-- Name: person_mention_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.person_mention_id_seq OWNED BY public.person_mention.id;


--
-- Name: post; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post (
    id integer NOT NULL,
    name character varying(200) NOT NULL,
    url text,
    body text,
    creator_id integer NOT NULL,
    community_id integer NOT NULL,
    removed boolean DEFAULT false NOT NULL,
    locked boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    deleted boolean DEFAULT false NOT NULL,
    nsfw boolean DEFAULT false NOT NULL,
    stickied boolean DEFAULT false NOT NULL,
    embed_title text,
    embed_description text,
    embed_html text,
    thumbnail_url text,
    ap_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    local boolean DEFAULT true NOT NULL
);


ALTER TABLE public.post OWNER TO owenyoung;

--
-- Name: post_aggregates; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post_aggregates (
    id integer NOT NULL,
    post_id integer NOT NULL,
    comments bigint DEFAULT 0 NOT NULL,
    score bigint DEFAULT 0 NOT NULL,
    upvotes bigint DEFAULT 0 NOT NULL,
    downvotes bigint DEFAULT 0 NOT NULL,
    stickied boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    newest_comment_time_necro timestamp without time zone DEFAULT now() NOT NULL,
    newest_comment_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.post_aggregates OWNER TO owenyoung;

--
-- Name: post_aggregates_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_aggregates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_aggregates_id_seq OWNER TO owenyoung;

--
-- Name: post_aggregates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_aggregates_id_seq OWNED BY public.post_aggregates.id;


--
-- Name: post_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_id_seq OWNER TO owenyoung;

--
-- Name: post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_id_seq OWNED BY public.post.id;


--
-- Name: post_like; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post_like (
    id integer NOT NULL,
    post_id integer NOT NULL,
    person_id integer NOT NULL,
    score smallint NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.post_like OWNER TO owenyoung;

--
-- Name: post_like_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_like_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_like_id_seq OWNER TO owenyoung;

--
-- Name: post_like_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_like_id_seq OWNED BY public.post_like.id;


--
-- Name: post_read; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post_read (
    id integer NOT NULL,
    post_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.post_read OWNER TO owenyoung;

--
-- Name: post_read_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_read_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_read_id_seq OWNER TO owenyoung;

--
-- Name: post_read_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_read_id_seq OWNED BY public.post_read.id;


--
-- Name: post_report; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post_report (
    id integer NOT NULL,
    creator_id integer NOT NULL,
    post_id integer NOT NULL,
    original_post_name character varying(100) NOT NULL,
    original_post_url text,
    original_post_body text,
    reason text NOT NULL,
    resolved boolean DEFAULT false NOT NULL,
    resolver_id integer,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone
);


ALTER TABLE public.post_report OWNER TO owenyoung;

--
-- Name: post_report_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_report_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_report_id_seq OWNER TO owenyoung;

--
-- Name: post_report_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_report_id_seq OWNED BY public.post_report.id;


--
-- Name: post_saved; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.post_saved (
    id integer NOT NULL,
    post_id integer NOT NULL,
    person_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.post_saved OWNER TO owenyoung;

--
-- Name: post_saved_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.post_saved_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_saved_id_seq OWNER TO owenyoung;

--
-- Name: post_saved_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.post_saved_id_seq OWNED BY public.post_saved.id;


--
-- Name: private_message; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.private_message (
    id integer NOT NULL,
    creator_id integer NOT NULL,
    recipient_id integer NOT NULL,
    content text NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    read boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    ap_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    local boolean DEFAULT true NOT NULL
);


ALTER TABLE public.private_message OWNER TO owenyoung;

--
-- Name: private_message_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.private_message_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.private_message_id_seq OWNER TO owenyoung;

--
-- Name: private_message_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.private_message_id_seq OWNED BY public.private_message.id;


--
-- Name: secret; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.secret (
    id integer NOT NULL,
    jwt_secret character varying DEFAULT gen_random_uuid() NOT NULL
);


ALTER TABLE public.secret OWNER TO owenyoung;

--
-- Name: secret_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.secret_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.secret_id_seq OWNER TO owenyoung;

--
-- Name: secret_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.secret_id_seq OWNED BY public.secret.id;


--
-- Name: site; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.site (
    id integer NOT NULL,
    name character varying(20) NOT NULL,
    sidebar text,
    creator_id integer NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    enable_downvotes boolean DEFAULT true NOT NULL,
    open_registration boolean DEFAULT true NOT NULL,
    enable_nsfw boolean DEFAULT true NOT NULL,
    icon text,
    banner text,
    description character varying(150),
    community_creation_admin_only boolean DEFAULT false NOT NULL
);


ALTER TABLE public.site OWNER TO owenyoung;

--
-- Name: site_aggregates; Type: TABLE; Schema: public; Owner: owenyoung
--

CREATE TABLE public.site_aggregates (
    id integer NOT NULL,
    site_id integer NOT NULL,
    users bigint DEFAULT 1 NOT NULL,
    posts bigint DEFAULT 0 NOT NULL,
    comments bigint DEFAULT 0 NOT NULL,
    communities bigint DEFAULT 0 NOT NULL,
    users_active_day bigint DEFAULT 0 NOT NULL,
    users_active_week bigint DEFAULT 0 NOT NULL,
    users_active_month bigint DEFAULT 0 NOT NULL,
    users_active_half_year bigint DEFAULT 0 NOT NULL
);


ALTER TABLE public.site_aggregates OWNER TO owenyoung;

--
-- Name: site_aggregates_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.site_aggregates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.site_aggregates_id_seq OWNER TO owenyoung;

--
-- Name: site_aggregates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.site_aggregates_id_seq OWNED BY public.site_aggregates.id;


--
-- Name: site_id_seq; Type: SEQUENCE; Schema: public; Owner: owenyoung
--

CREATE SEQUENCE public.site_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.site_id_seq OWNER TO owenyoung;

--
-- Name: site_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: owenyoung
--

ALTER SEQUENCE public.site_id_seq OWNED BY public.site.id;


--
-- Name: deps_saved_ddl; Type: TABLE; Schema: utils; Owner: owenyoung
--

CREATE TABLE utils.deps_saved_ddl (
    id integer NOT NULL,
    view_schema character varying(255),
    view_name character varying(255),
    ddl_to_run text
);


ALTER TABLE utils.deps_saved_ddl OWNER TO owenyoung;

--
-- Name: deps_saved_ddl_id_seq; Type: SEQUENCE; Schema: utils; Owner: owenyoung
--

CREATE SEQUENCE utils.deps_saved_ddl_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE utils.deps_saved_ddl_id_seq OWNER TO owenyoung;

--
-- Name: deps_saved_ddl_id_seq; Type: SEQUENCE OWNED BY; Schema: utils; Owner: owenyoung
--

ALTER SEQUENCE utils.deps_saved_ddl_id_seq OWNED BY utils.deps_saved_ddl.id;


--
-- Name: activity id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.activity ALTER COLUMN id SET DEFAULT nextval('public.activity_id_seq'::regclass);


--
-- Name: comment id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment ALTER COLUMN id SET DEFAULT nextval('public.comment_id_seq'::regclass);


--
-- Name: comment_aggregates id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_aggregates ALTER COLUMN id SET DEFAULT nextval('public.comment_aggregates_id_seq'::regclass);


--
-- Name: comment_like id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like ALTER COLUMN id SET DEFAULT nextval('public.comment_like_id_seq'::regclass);


--
-- Name: comment_report id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report ALTER COLUMN id SET DEFAULT nextval('public.comment_report_id_seq'::regclass);


--
-- Name: comment_saved id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_saved ALTER COLUMN id SET DEFAULT nextval('public.comment_saved_id_seq'::regclass);


--
-- Name: community id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community ALTER COLUMN id SET DEFAULT nextval('public.community_id_seq'::regclass);


--
-- Name: community_aggregates id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_aggregates ALTER COLUMN id SET DEFAULT nextval('public.community_aggregates_id_seq'::regclass);


--
-- Name: community_block id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_block ALTER COLUMN id SET DEFAULT nextval('public.community_block_id_seq'::regclass);


--
-- Name: community_follower id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_follower ALTER COLUMN id SET DEFAULT nextval('public.community_follower_id_seq'::regclass);


--
-- Name: community_moderator id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_moderator ALTER COLUMN id SET DEFAULT nextval('public.community_moderator_id_seq'::regclass);


--
-- Name: community_person_ban id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_person_ban ALTER COLUMN id SET DEFAULT nextval('public.community_person_ban_id_seq'::regclass);


--
-- Name: local_user id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.local_user ALTER COLUMN id SET DEFAULT nextval('public.local_user_id_seq'::regclass);


--
-- Name: mod_add id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add ALTER COLUMN id SET DEFAULT nextval('public.mod_add_id_seq'::regclass);


--
-- Name: mod_add_community id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add_community ALTER COLUMN id SET DEFAULT nextval('public.mod_add_community_id_seq'::regclass);


--
-- Name: mod_ban id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban ALTER COLUMN id SET DEFAULT nextval('public.mod_ban_id_seq'::regclass);


--
-- Name: mod_ban_from_community id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban_from_community ALTER COLUMN id SET DEFAULT nextval('public.mod_ban_from_community_id_seq'::regclass);


--
-- Name: mod_lock_post id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_lock_post ALTER COLUMN id SET DEFAULT nextval('public.mod_lock_post_id_seq'::regclass);


--
-- Name: mod_remove_comment id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_comment ALTER COLUMN id SET DEFAULT nextval('public.mod_remove_comment_id_seq'::regclass);


--
-- Name: mod_remove_community id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_community ALTER COLUMN id SET DEFAULT nextval('public.mod_remove_community_id_seq'::regclass);


--
-- Name: mod_remove_post id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_post ALTER COLUMN id SET DEFAULT nextval('public.mod_remove_post_id_seq'::regclass);


--
-- Name: mod_sticky_post id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_sticky_post ALTER COLUMN id SET DEFAULT nextval('public.mod_sticky_post_id_seq'::regclass);


--
-- Name: mod_transfer_community id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_transfer_community ALTER COLUMN id SET DEFAULT nextval('public.mod_transfer_community_id_seq'::regclass);


--
-- Name: password_reset_request id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.password_reset_request ALTER COLUMN id SET DEFAULT nextval('public.password_reset_request_id_seq'::regclass);


--
-- Name: person id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person ALTER COLUMN id SET DEFAULT nextval('public.person_id_seq'::regclass);


--
-- Name: person_aggregates id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_aggregates ALTER COLUMN id SET DEFAULT nextval('public.person_aggregates_id_seq'::regclass);


--
-- Name: person_ban id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_ban ALTER COLUMN id SET DEFAULT nextval('public.person_ban_id_seq'::regclass);


--
-- Name: person_block id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_block ALTER COLUMN id SET DEFAULT nextval('public.person_block_id_seq'::regclass);


--
-- Name: person_mention id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_mention ALTER COLUMN id SET DEFAULT nextval('public.person_mention_id_seq'::regclass);


--
-- Name: post id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post ALTER COLUMN id SET DEFAULT nextval('public.post_id_seq'::regclass);


--
-- Name: post_aggregates id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_aggregates ALTER COLUMN id SET DEFAULT nextval('public.post_aggregates_id_seq'::regclass);


--
-- Name: post_like id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_like ALTER COLUMN id SET DEFAULT nextval('public.post_like_id_seq'::regclass);


--
-- Name: post_read id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_read ALTER COLUMN id SET DEFAULT nextval('public.post_read_id_seq'::regclass);


--
-- Name: post_report id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report ALTER COLUMN id SET DEFAULT nextval('public.post_report_id_seq'::regclass);


--
-- Name: post_saved id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_saved ALTER COLUMN id SET DEFAULT nextval('public.post_saved_id_seq'::regclass);


--
-- Name: private_message id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.private_message ALTER COLUMN id SET DEFAULT nextval('public.private_message_id_seq'::regclass);


--
-- Name: secret id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.secret ALTER COLUMN id SET DEFAULT nextval('public.secret_id_seq'::regclass);


--
-- Name: site id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site ALTER COLUMN id SET DEFAULT nextval('public.site_id_seq'::regclass);


--
-- Name: site_aggregates id; Type: DEFAULT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site_aggregates ALTER COLUMN id SET DEFAULT nextval('public.site_aggregates_id_seq'::regclass);


--
-- Name: deps_saved_ddl id; Type: DEFAULT; Schema: utils; Owner: owenyoung
--

ALTER TABLE ONLY utils.deps_saved_ddl ALTER COLUMN id SET DEFAULT nextval('utils.deps_saved_ddl_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2021-10-13 09:16:54.814596
20190226002946	2021-10-13 09:16:54.854908
20190227170003	2021-10-13 09:16:54.865128
20190303163336	2021-10-13 09:16:54.887225
20190305233828	2021-10-13 09:16:54.906086
20190330212058	2021-10-13 09:16:54.91952
20190403155205	2021-10-13 09:16:54.926511
20190403155309	2021-10-13 09:16:54.932834
20190407003142	2021-10-13 09:16:54.937479
20190408015947	2021-10-13 09:16:54.964856
20190411144915	2021-10-13 09:16:54.968983
20190429175834	2021-10-13 09:16:54.976465
20190502051656	2021-10-13 09:16:54.989956
20190601222649	2021-10-13 09:16:54.993405
20190811000918	2021-10-13 09:16:54.998447
20190829040006	2021-10-13 09:16:55.005489
20190905230317	2021-10-13 09:16:55.007408
20190909042010	2021-10-13 09:16:55.011885
20191015181630	2021-10-13 09:16:55.020096
20191019052737	2021-10-13 09:16:55.021266
20191021011237	2021-10-13 09:16:55.026677
20191024002614	2021-10-13 09:16:55.028013
20191209060754	2021-10-13 09:16:55.032132
20191211181820	2021-10-13 09:16:55.033279
20191229164820	2021-10-13 09:16:55.035945
20200101200418	2021-10-13 09:16:55.060795
20200102172755	2021-10-13 09:16:55.063202
20200111012452	2021-10-13 09:16:55.065739
20200113025151	2021-10-13 09:16:55.077696
20200121001001	2021-10-13 09:16:55.124658
20200129011901	2021-10-13 09:16:55.140492
20200129030825	2021-10-13 09:16:55.143526
20200202004806	2021-10-13 09:16:55.147185
20200206165953	2021-10-13 09:16:55.184629
20200207210055	2021-10-13 09:16:55.205095
20200208145624	2021-10-13 09:16:55.224819
20200306202329	2021-10-13 09:16:55.240963
20200326192410	2021-10-13 09:16:55.258523
20200403194936	2021-10-13 09:16:55.265708
20200407135912	2021-10-13 09:16:55.267681
20200414163701	2021-10-13 09:16:55.275467
20200421123957	2021-10-13 09:16:55.340779
20200505210233	2021-10-13 09:16:55.343083
20200630135809	2021-10-13 09:16:55.349921
20200708202609	2021-10-13 09:16:55.415727
20200712100442	2021-10-13 09:16:55.45716
20200718234519	2021-10-13 09:16:55.483313
20200803000110	2021-10-13 09:16:55.487793
20200806205355	2021-10-13 09:16:55.560394
20200825132005	2021-10-13 09:16:55.573242
20200907231141	2021-10-13 09:16:55.579911
20201007234221	2021-10-13 09:16:55.583802
20201010035723	2021-10-13 09:16:55.585441
20201013212240	2021-10-13 09:16:55.586457
20201023115011	2021-10-13 09:16:55.601484
20201105152724	2021-10-13 09:16:55.603166
20201110150835	2021-10-13 09:16:55.604489
20201126134531	2021-10-13 09:16:55.605428
20201202152437	2021-10-13 09:16:55.606321
20201203035643	2021-10-13 09:16:55.612154
20201204183345	2021-10-13 09:16:55.619308
20201210152350	2021-10-13 09:16:55.625757
20201214020038	2021-10-13 09:16:55.632459
20201217030456	2021-10-13 09:16:55.637901
20201217031053	2021-10-13 09:16:55.642049
20210105200932	2021-10-13 09:16:55.658401
20210126173850	2021-10-13 09:16:55.67408
20210127202728	2021-10-13 09:16:55.674939
20210131050334	2021-10-13 09:16:55.679782
20210202153240	2021-10-13 09:16:55.682005
20210210164051	2021-10-13 09:16:55.71133
20210213210612	2021-10-13 09:16:55.716038
20210225112959	2021-10-13 09:16:55.717146
20210228162616	2021-10-13 09:16:55.719476
20210304040229	2021-10-13 09:16:55.720296
20210309171136	2021-10-13 09:16:55.722395
20210319014144	2021-10-13 09:16:55.740295
20210320185321	2021-10-13 09:16:55.741513
20210331103917	2021-10-13 09:16:55.745649
20210331105915	2021-10-13 09:16:55.746566
20210331144349	2021-10-13 09:16:55.749681
20210401173552	2021-10-13 09:16:55.750529
20210401181826	2021-10-13 09:16:55.753498
20210402021422	2021-10-13 09:16:55.754749
20210420155001	2021-10-13 09:16:55.755946
20210424174047	2021-10-13 09:16:55.756615
20210719130929	2021-10-13 09:16:55.757323
20210720102033	2021-10-13 09:16:55.757996
20210802002342	2021-10-13 09:16:55.761369
20210804223559	2021-10-13 09:16:55.762655
20210816004209	2021-10-13 09:16:55.769717
20210817210508	2021-10-13 09:16:55.77113
20210920112945	2021-10-13 09:16:55.774966
\.


--
-- Data for Name: activity; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.activity (id, data, local, published, updated, ap_id, sensitive) FROM stdin;
\.


--
-- Data for Name: comment; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.comment (id, creator_id, post_id, parent_id, content, removed, read, published, updated, deleted, ap_id, local) FROM stdin;
\.


--
-- Data for Name: comment_aggregates; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.comment_aggregates (id, comment_id, score, upvotes, downvotes, published) FROM stdin;
\.


--
-- Data for Name: comment_like; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.comment_like (id, person_id, comment_id, post_id, score, published) FROM stdin;
\.


--
-- Data for Name: comment_report; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.comment_report (id, creator_id, comment_id, original_comment_text, reason, resolved, resolver_id, published, updated) FROM stdin;
\.


--
-- Data for Name: comment_saved; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.comment_saved (id, comment_id, person_id, published) FROM stdin;
\.


--
-- Data for Name: community; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community (id, name, title, description, removed, published, updated, deleted, nsfw, actor_id, local, private_key, public_key, last_refreshed_at, icon, banner, followers_url, inbox_url, shared_inbox_url) FROM stdin;
\.


--
-- Data for Name: community_aggregates; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community_aggregates (id, community_id, subscribers, posts, comments, published, users_active_day, users_active_week, users_active_month, users_active_half_year) FROM stdin;
\.


--
-- Data for Name: community_block; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community_block (id, person_id, community_id, published) FROM stdin;
\.


--
-- Data for Name: community_follower; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community_follower (id, community_id, person_id, published, pending) FROM stdin;
\.


--
-- Data for Name: community_moderator; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community_moderator (id, community_id, person_id, published) FROM stdin;
\.


--
-- Data for Name: community_person_ban; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.community_person_ban (id, community_id, person_id, published) FROM stdin;
\.


--
-- Data for Name: local_user; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.local_user (id, person_id, password_encrypted, email, show_nsfw, theme, default_sort_type, default_listing_type, lang, show_avatars, send_notifications_to_email, validator_time, show_scores, show_bot_accounts, show_read_posts, show_new_post_notifs) FROM stdin;
\.


--
-- Data for Name: mod_add; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_add (id, mod_person_id, other_person_id, removed, when_) FROM stdin;
\.


--
-- Data for Name: mod_add_community; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_add_community (id, mod_person_id, other_person_id, community_id, removed, when_) FROM stdin;
\.


--
-- Data for Name: mod_ban; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_ban (id, mod_person_id, other_person_id, reason, banned, expires, when_) FROM stdin;
\.


--
-- Data for Name: mod_ban_from_community; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_ban_from_community (id, mod_person_id, other_person_id, community_id, reason, banned, expires, when_) FROM stdin;
\.


--
-- Data for Name: mod_lock_post; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_lock_post (id, mod_person_id, post_id, locked, when_) FROM stdin;
\.


--
-- Data for Name: mod_remove_comment; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_remove_comment (id, mod_person_id, comment_id, reason, removed, when_) FROM stdin;
\.


--
-- Data for Name: mod_remove_community; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_remove_community (id, mod_person_id, community_id, reason, removed, expires, when_) FROM stdin;
\.


--
-- Data for Name: mod_remove_post; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_remove_post (id, mod_person_id, post_id, reason, removed, when_) FROM stdin;
\.


--
-- Data for Name: mod_sticky_post; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_sticky_post (id, mod_person_id, post_id, stickied, when_) FROM stdin;
\.


--
-- Data for Name: mod_transfer_community; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.mod_transfer_community (id, mod_person_id, other_person_id, community_id, removed, when_) FROM stdin;
\.


--
-- Data for Name: password_reset_request; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.password_reset_request (id, token_encrypted, published, local_user_id) FROM stdin;
\.


--
-- Data for Name: person; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.person (id, name, display_name, avatar, banned, published, updated, actor_id, bio, local, private_key, public_key, last_refreshed_at, banner, deleted, inbox_url, shared_inbox_url, matrix_user_id, admin, bot_account) FROM stdin;
\.


--
-- Data for Name: person_aggregates; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.person_aggregates (id, person_id, post_count, post_score, comment_count, comment_score) FROM stdin;
\.


--
-- Data for Name: person_ban; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.person_ban (id, person_id, published) FROM stdin;
\.


--
-- Data for Name: person_block; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.person_block (id, person_id, target_id, published) FROM stdin;
\.


--
-- Data for Name: person_mention; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.person_mention (id, recipient_id, comment_id, read, published) FROM stdin;
\.


--
-- Data for Name: post; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post (id, name, url, body, creator_id, community_id, removed, locked, published, updated, deleted, nsfw, stickied, embed_title, embed_description, embed_html, thumbnail_url, ap_id, local) FROM stdin;
\.


--
-- Data for Name: post_aggregates; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post_aggregates (id, post_id, comments, score, upvotes, downvotes, stickied, published, newest_comment_time_necro, newest_comment_time) FROM stdin;
\.


--
-- Data for Name: post_like; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post_like (id, post_id, person_id, score, published) FROM stdin;
\.


--
-- Data for Name: post_read; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post_read (id, post_id, person_id, published) FROM stdin;
\.


--
-- Data for Name: post_report; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post_report (id, creator_id, post_id, original_post_name, original_post_url, original_post_body, reason, resolved, resolver_id, published, updated) FROM stdin;
\.


--
-- Data for Name: post_saved; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.post_saved (id, post_id, person_id, published) FROM stdin;
\.


--
-- Data for Name: private_message; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.private_message (id, creator_id, recipient_id, content, deleted, read, published, updated, ap_id, local) FROM stdin;
\.


--
-- Data for Name: secret; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.secret (id, jwt_secret) FROM stdin;
1	61d85a17-23fb-40d3-b82c-4d08047e4ef8
\.


--
-- Data for Name: site; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.site (id, name, sidebar, creator_id, published, updated, enable_downvotes, open_registration, enable_nsfw, icon, banner, description, community_creation_admin_only) FROM stdin;
\.


--
-- Data for Name: site_aggregates; Type: TABLE DATA; Schema: public; Owner: owenyoung
--

COPY public.site_aggregates (id, site_id, users, posts, comments, communities, users_active_day, users_active_week, users_active_month, users_active_half_year) FROM stdin;
\.


--
-- Data for Name: deps_saved_ddl; Type: TABLE DATA; Schema: utils; Owner: owenyoung
--

COPY utils.deps_saved_ddl (id, view_schema, view_name, ddl_to_run) FROM stdin;
\.


--
-- Name: activity_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.activity_id_seq', 1, false);


--
-- Name: comment_aggregates_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.comment_aggregates_id_seq', 1, false);


--
-- Name: comment_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.comment_id_seq', 1, false);


--
-- Name: comment_like_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.comment_like_id_seq', 1, false);


--
-- Name: comment_report_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.comment_report_id_seq', 1, false);


--
-- Name: comment_saved_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.comment_saved_id_seq', 1, false);


--
-- Name: community_aggregates_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_aggregates_id_seq', 1, false);


--
-- Name: community_block_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_block_id_seq', 1, false);


--
-- Name: community_follower_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_follower_id_seq', 1, false);


--
-- Name: community_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_id_seq', 1, true);


--
-- Name: community_moderator_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_moderator_id_seq', 1, false);


--
-- Name: community_person_ban_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.community_person_ban_id_seq', 1, false);


--
-- Name: local_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.local_user_id_seq', 1, false);


--
-- Name: mod_add_community_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_add_community_id_seq', 1, false);


--
-- Name: mod_add_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_add_id_seq', 1, false);


--
-- Name: mod_ban_from_community_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_ban_from_community_id_seq', 1, false);


--
-- Name: mod_ban_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_ban_id_seq', 1, false);


--
-- Name: mod_lock_post_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_lock_post_id_seq', 1, false);


--
-- Name: mod_remove_comment_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_remove_comment_id_seq', 1, false);


--
-- Name: mod_remove_community_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_remove_community_id_seq', 1, false);


--
-- Name: mod_remove_post_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_remove_post_id_seq', 1, false);


--
-- Name: mod_sticky_post_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_sticky_post_id_seq', 1, false);


--
-- Name: mod_transfer_community_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.mod_transfer_community_id_seq', 1, false);


--
-- Name: password_reset_request_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.password_reset_request_id_seq', 1, false);


--
-- Name: person_aggregates_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.person_aggregates_id_seq', 1, false);


--
-- Name: person_ban_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.person_ban_id_seq', 1, false);


--
-- Name: person_block_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.person_block_id_seq', 1, false);


--
-- Name: person_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.person_id_seq', 1, true);


--
-- Name: person_mention_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.person_mention_id_seq', 1, false);


--
-- Name: post_aggregates_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_aggregates_id_seq', 1, false);


--
-- Name: post_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_id_seq', 1, false);


--
-- Name: post_like_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_like_id_seq', 1, false);


--
-- Name: post_read_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_read_id_seq', 1, false);


--
-- Name: post_report_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_report_id_seq', 1, false);


--
-- Name: post_saved_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.post_saved_id_seq', 1, false);


--
-- Name: private_message_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.private_message_id_seq', 1, false);


--
-- Name: secret_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.secret_id_seq', 1, true);


--
-- Name: site_aggregates_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.site_aggregates_id_seq', 1, false);


--
-- Name: site_id_seq; Type: SEQUENCE SET; Schema: public; Owner: owenyoung
--

SELECT pg_catalog.setval('public.site_id_seq', 1, false);


--
-- Name: deps_saved_ddl_id_seq; Type: SEQUENCE SET; Schema: utils; Owner: owenyoung
--

SELECT pg_catalog.setval('utils.deps_saved_ddl_id_seq', 1, false);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: activity activity_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.activity
    ADD CONSTRAINT activity_pkey PRIMARY KEY (id);


--
-- Name: comment_aggregates comment_aggregates_comment_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_aggregates
    ADD CONSTRAINT comment_aggregates_comment_id_key UNIQUE (comment_id);


--
-- Name: comment_aggregates comment_aggregates_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_aggregates
    ADD CONSTRAINT comment_aggregates_pkey PRIMARY KEY (id);


--
-- Name: comment_like comment_like_comment_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like
    ADD CONSTRAINT comment_like_comment_id_person_id_key UNIQUE (comment_id, person_id);


--
-- Name: comment_like comment_like_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like
    ADD CONSTRAINT comment_like_pkey PRIMARY KEY (id);


--
-- Name: comment comment_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment
    ADD CONSTRAINT comment_pkey PRIMARY KEY (id);


--
-- Name: comment_report comment_report_comment_id_creator_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report
    ADD CONSTRAINT comment_report_comment_id_creator_id_key UNIQUE (comment_id, creator_id);


--
-- Name: comment_report comment_report_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report
    ADD CONSTRAINT comment_report_pkey PRIMARY KEY (id);


--
-- Name: comment_saved comment_saved_comment_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_saved
    ADD CONSTRAINT comment_saved_comment_id_person_id_key UNIQUE (comment_id, person_id);


--
-- Name: comment_saved comment_saved_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_saved
    ADD CONSTRAINT comment_saved_pkey PRIMARY KEY (id);


--
-- Name: community_aggregates community_aggregates_community_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_aggregates
    ADD CONSTRAINT community_aggregates_community_id_key UNIQUE (community_id);


--
-- Name: community_aggregates community_aggregates_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_aggregates
    ADD CONSTRAINT community_aggregates_pkey PRIMARY KEY (id);


--
-- Name: community_block community_block_person_id_community_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_block
    ADD CONSTRAINT community_block_person_id_community_id_key UNIQUE (person_id, community_id);


--
-- Name: community_block community_block_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_block
    ADD CONSTRAINT community_block_pkey PRIMARY KEY (id);


--
-- Name: community_follower community_follower_community_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_follower
    ADD CONSTRAINT community_follower_community_id_person_id_key UNIQUE (community_id, person_id);


--
-- Name: community_follower community_follower_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_follower
    ADD CONSTRAINT community_follower_pkey PRIMARY KEY (id);


--
-- Name: community_moderator community_moderator_community_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_moderator
    ADD CONSTRAINT community_moderator_community_id_person_id_key UNIQUE (community_id, person_id);


--
-- Name: community_moderator community_moderator_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_moderator
    ADD CONSTRAINT community_moderator_pkey PRIMARY KEY (id);


--
-- Name: community_person_ban community_person_ban_community_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_person_ban
    ADD CONSTRAINT community_person_ban_community_id_person_id_key UNIQUE (community_id, person_id);


--
-- Name: community_person_ban community_person_ban_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_person_ban
    ADD CONSTRAINT community_person_ban_pkey PRIMARY KEY (id);


--
-- Name: community community_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community
    ADD CONSTRAINT community_pkey PRIMARY KEY (id);


--
-- Name: comment idx_comment_ap_id; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment
    ADD CONSTRAINT idx_comment_ap_id UNIQUE (ap_id);


--
-- Name: community idx_community_actor_id; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community
    ADD CONSTRAINT idx_community_actor_id UNIQUE (actor_id);


--
-- Name: community idx_community_followers_url; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community
    ADD CONSTRAINT idx_community_followers_url UNIQUE (followers_url);


--
-- Name: community idx_community_inbox_url; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community
    ADD CONSTRAINT idx_community_inbox_url UNIQUE (inbox_url);


--
-- Name: person idx_person_actor_id; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person
    ADD CONSTRAINT idx_person_actor_id UNIQUE (actor_id);


--
-- Name: person idx_person_inbox_url; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person
    ADD CONSTRAINT idx_person_inbox_url UNIQUE (inbox_url);


--
-- Name: post idx_post_ap_id; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT idx_post_ap_id UNIQUE (ap_id);


--
-- Name: private_message idx_private_message_ap_id; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.private_message
    ADD CONSTRAINT idx_private_message_ap_id UNIQUE (ap_id);


--
-- Name: local_user local_user_email_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.local_user
    ADD CONSTRAINT local_user_email_key UNIQUE (email);


--
-- Name: local_user local_user_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.local_user
    ADD CONSTRAINT local_user_person_id_key UNIQUE (person_id);


--
-- Name: local_user local_user_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.local_user
    ADD CONSTRAINT local_user_pkey PRIMARY KEY (id);


--
-- Name: mod_add_community mod_add_community_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add_community
    ADD CONSTRAINT mod_add_community_pkey PRIMARY KEY (id);


--
-- Name: mod_add mod_add_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add
    ADD CONSTRAINT mod_add_pkey PRIMARY KEY (id);


--
-- Name: mod_ban_from_community mod_ban_from_community_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban_from_community
    ADD CONSTRAINT mod_ban_from_community_pkey PRIMARY KEY (id);


--
-- Name: mod_ban mod_ban_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban
    ADD CONSTRAINT mod_ban_pkey PRIMARY KEY (id);


--
-- Name: mod_lock_post mod_lock_post_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_lock_post
    ADD CONSTRAINT mod_lock_post_pkey PRIMARY KEY (id);


--
-- Name: mod_remove_comment mod_remove_comment_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_comment
    ADD CONSTRAINT mod_remove_comment_pkey PRIMARY KEY (id);


--
-- Name: mod_remove_community mod_remove_community_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_community
    ADD CONSTRAINT mod_remove_community_pkey PRIMARY KEY (id);


--
-- Name: mod_remove_post mod_remove_post_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_post
    ADD CONSTRAINT mod_remove_post_pkey PRIMARY KEY (id);


--
-- Name: mod_sticky_post mod_sticky_post_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_sticky_post
    ADD CONSTRAINT mod_sticky_post_pkey PRIMARY KEY (id);


--
-- Name: mod_transfer_community mod_transfer_community_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_transfer_community
    ADD CONSTRAINT mod_transfer_community_pkey PRIMARY KEY (id);


--
-- Name: password_reset_request password_reset_request_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.password_reset_request
    ADD CONSTRAINT password_reset_request_pkey PRIMARY KEY (id);


--
-- Name: person person__pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person
    ADD CONSTRAINT person__pkey PRIMARY KEY (id);


--
-- Name: person_aggregates person_aggregates_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_aggregates
    ADD CONSTRAINT person_aggregates_person_id_key UNIQUE (person_id);


--
-- Name: person_aggregates person_aggregates_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_aggregates
    ADD CONSTRAINT person_aggregates_pkey PRIMARY KEY (id);


--
-- Name: person_ban person_ban_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_ban
    ADD CONSTRAINT person_ban_person_id_key UNIQUE (person_id);


--
-- Name: person_ban person_ban_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_ban
    ADD CONSTRAINT person_ban_pkey PRIMARY KEY (id);


--
-- Name: person_block person_block_person_id_target_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_block
    ADD CONSTRAINT person_block_person_id_target_id_key UNIQUE (person_id, target_id);


--
-- Name: person_block person_block_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_block
    ADD CONSTRAINT person_block_pkey PRIMARY KEY (id);


--
-- Name: person_mention person_mention_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_mention
    ADD CONSTRAINT person_mention_pkey PRIMARY KEY (id);


--
-- Name: person_mention person_mention_recipient_id_comment_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_mention
    ADD CONSTRAINT person_mention_recipient_id_comment_id_key UNIQUE (recipient_id, comment_id);


--
-- Name: post_aggregates post_aggregates_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_aggregates
    ADD CONSTRAINT post_aggregates_pkey PRIMARY KEY (id);


--
-- Name: post_aggregates post_aggregates_post_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_aggregates
    ADD CONSTRAINT post_aggregates_post_id_key UNIQUE (post_id);


--
-- Name: post_like post_like_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT post_like_pkey PRIMARY KEY (id);


--
-- Name: post_like post_like_post_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT post_like_post_id_person_id_key UNIQUE (post_id, person_id);


--
-- Name: post post_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_pkey PRIMARY KEY (id);


--
-- Name: post_read post_read_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_read
    ADD CONSTRAINT post_read_pkey PRIMARY KEY (id);


--
-- Name: post_read post_read_post_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_read
    ADD CONSTRAINT post_read_post_id_person_id_key UNIQUE (post_id, person_id);


--
-- Name: post_report post_report_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report
    ADD CONSTRAINT post_report_pkey PRIMARY KEY (id);


--
-- Name: post_report post_report_post_id_creator_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report
    ADD CONSTRAINT post_report_post_id_creator_id_key UNIQUE (post_id, creator_id);


--
-- Name: post_saved post_saved_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_saved
    ADD CONSTRAINT post_saved_pkey PRIMARY KEY (id);


--
-- Name: post_saved post_saved_post_id_person_id_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_saved
    ADD CONSTRAINT post_saved_post_id_person_id_key UNIQUE (post_id, person_id);


--
-- Name: private_message private_message_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.private_message
    ADD CONSTRAINT private_message_pkey PRIMARY KEY (id);


--
-- Name: secret secret_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.secret
    ADD CONSTRAINT secret_pkey PRIMARY KEY (id);


--
-- Name: site_aggregates site_aggregates_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site_aggregates
    ADD CONSTRAINT site_aggregates_pkey PRIMARY KEY (id);


--
-- Name: site site_name_key; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site
    ADD CONSTRAINT site_name_key UNIQUE (name);


--
-- Name: site site_pkey; Type: CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site
    ADD CONSTRAINT site_pkey PRIMARY KEY (id);


--
-- Name: deps_saved_ddl deps_saved_ddl_pkey; Type: CONSTRAINT; Schema: utils; Owner: owenyoung
--

ALTER TABLE ONLY utils.deps_saved_ddl
    ADD CONSTRAINT deps_saved_ddl_pkey PRIMARY KEY (id);


--
-- Name: idx_activity_unique_apid; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE UNIQUE INDEX idx_activity_unique_apid ON public.activity USING btree (((data ->> 'id'::text)));


--
-- Name: idx_comment_aggregates_hot; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_aggregates_hot ON public.comment_aggregates USING btree (public.hot_rank((score)::numeric, published) DESC, published DESC);


--
-- Name: idx_comment_aggregates_score; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_aggregates_score ON public.comment_aggregates USING btree (score DESC);


--
-- Name: idx_comment_creator; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_creator ON public.comment USING btree (creator_id);


--
-- Name: idx_comment_like_comment; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_like_comment ON public.comment_like USING btree (comment_id);


--
-- Name: idx_comment_like_person; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_like_person ON public.comment_like USING btree (person_id);


--
-- Name: idx_comment_like_post; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_like_post ON public.comment_like USING btree (post_id);


--
-- Name: idx_comment_parent; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_parent ON public.comment USING btree (parent_id);


--
-- Name: idx_comment_post; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_post ON public.comment USING btree (post_id);


--
-- Name: idx_comment_published; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_comment_published ON public.comment USING btree (published DESC);


--
-- Name: idx_community_aggregates_hot; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_community_aggregates_hot ON public.community_aggregates USING btree (public.hot_rank((subscribers)::numeric, published) DESC, published DESC);


--
-- Name: idx_community_aggregates_subscribers; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_community_aggregates_subscribers ON public.community_aggregates USING btree (subscribers DESC);


--
-- Name: idx_community_aggregates_users_active_month; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_community_aggregates_users_active_month ON public.community_aggregates USING btree (users_active_month DESC);


--
-- Name: idx_community_lower_actor_id; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE UNIQUE INDEX idx_community_lower_actor_id ON public.community USING btree (lower((actor_id)::text));


--
-- Name: idx_community_published; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_community_published ON public.community USING btree (published DESC);


--
-- Name: idx_person_aggregates_comment_score; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_person_aggregates_comment_score ON public.person_aggregates USING btree (comment_score DESC);


--
-- Name: idx_person_lower_actor_id; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE UNIQUE INDEX idx_person_lower_actor_id ON public.person USING btree (lower((actor_id)::text));


--
-- Name: idx_person_published; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_person_published ON public.person USING btree (published DESC);


--
-- Name: idx_post_aggregates_active; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_active ON public.post_aggregates USING btree (public.hot_rank((score)::numeric, newest_comment_time_necro) DESC, newest_comment_time_necro DESC);


--
-- Name: idx_post_aggregates_comments; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_comments ON public.post_aggregates USING btree (comments DESC);


--
-- Name: idx_post_aggregates_hot; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_hot ON public.post_aggregates USING btree (public.hot_rank((score)::numeric, published) DESC, published DESC);


--
-- Name: idx_post_aggregates_newest_comment_time; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_newest_comment_time ON public.post_aggregates USING btree (newest_comment_time DESC);


--
-- Name: idx_post_aggregates_published; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_published ON public.post_aggregates USING btree (published DESC);


--
-- Name: idx_post_aggregates_score; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_score ON public.post_aggregates USING btree (score DESC);


--
-- Name: idx_post_aggregates_stickied_active; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_active ON public.post_aggregates USING btree (stickied DESC, public.hot_rank((score)::numeric, newest_comment_time_necro) DESC, newest_comment_time_necro DESC);


--
-- Name: idx_post_aggregates_stickied_comments; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_comments ON public.post_aggregates USING btree (stickied DESC, comments DESC);


--
-- Name: idx_post_aggregates_stickied_hot; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_hot ON public.post_aggregates USING btree (stickied DESC, public.hot_rank((score)::numeric, published) DESC, published DESC);


--
-- Name: idx_post_aggregates_stickied_newest_comment_time; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_newest_comment_time ON public.post_aggregates USING btree (stickied DESC, newest_comment_time DESC);


--
-- Name: idx_post_aggregates_stickied_published; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_published ON public.post_aggregates USING btree (stickied DESC, published DESC);


--
-- Name: idx_post_aggregates_stickied_score; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_aggregates_stickied_score ON public.post_aggregates USING btree (stickied DESC, score DESC);


--
-- Name: idx_post_community; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_community ON public.post USING btree (community_id);


--
-- Name: idx_post_creator; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_creator ON public.post USING btree (creator_id);


--
-- Name: idx_post_like_person; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_like_person ON public.post_like USING btree (person_id);


--
-- Name: idx_post_like_post; Type: INDEX; Schema: public; Owner: owenyoung
--

CREATE INDEX idx_post_like_post ON public.post_like USING btree (post_id);


--
-- Name: comment comment_aggregates_comment; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER comment_aggregates_comment AFTER INSERT OR DELETE ON public.comment FOR EACH ROW EXECUTE FUNCTION public.comment_aggregates_comment();


--
-- Name: comment_like comment_aggregates_score; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER comment_aggregates_score AFTER INSERT OR DELETE ON public.comment_like FOR EACH ROW EXECUTE FUNCTION public.comment_aggregates_score();


--
-- Name: comment community_aggregates_comment_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER community_aggregates_comment_count AFTER INSERT OR DELETE ON public.comment FOR EACH ROW EXECUTE FUNCTION public.community_aggregates_comment_count();


--
-- Name: community community_aggregates_community; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER community_aggregates_community AFTER INSERT OR DELETE ON public.community FOR EACH ROW EXECUTE FUNCTION public.community_aggregates_community();


--
-- Name: post community_aggregates_post_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER community_aggregates_post_count AFTER INSERT OR DELETE ON public.post FOR EACH ROW EXECUTE FUNCTION public.community_aggregates_post_count();


--
-- Name: community_follower community_aggregates_subscriber_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER community_aggregates_subscriber_count AFTER INSERT OR DELETE ON public.community_follower FOR EACH ROW EXECUTE FUNCTION public.community_aggregates_subscriber_count();


--
-- Name: comment person_aggregates_comment_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER person_aggregates_comment_count AFTER INSERT OR DELETE ON public.comment FOR EACH ROW EXECUTE FUNCTION public.person_aggregates_comment_count();


--
-- Name: comment_like person_aggregates_comment_score; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER person_aggregates_comment_score AFTER INSERT OR DELETE ON public.comment_like FOR EACH ROW EXECUTE FUNCTION public.person_aggregates_comment_score();


--
-- Name: person person_aggregates_person; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER person_aggregates_person AFTER INSERT OR DELETE ON public.person FOR EACH ROW EXECUTE FUNCTION public.person_aggregates_person();


--
-- Name: post person_aggregates_post_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER person_aggregates_post_count AFTER INSERT OR DELETE ON public.post FOR EACH ROW EXECUTE FUNCTION public.person_aggregates_post_count();


--
-- Name: post_like person_aggregates_post_score; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER person_aggregates_post_score AFTER INSERT OR DELETE ON public.post_like FOR EACH ROW EXECUTE FUNCTION public.person_aggregates_post_score();


--
-- Name: comment post_aggregates_comment_count; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER post_aggregates_comment_count AFTER INSERT OR DELETE ON public.comment FOR EACH ROW EXECUTE FUNCTION public.post_aggregates_comment_count();


--
-- Name: comment post_aggregates_comment_set_deleted; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER post_aggregates_comment_set_deleted AFTER UPDATE OF deleted ON public.comment FOR EACH ROW EXECUTE FUNCTION public.post_aggregates_comment_deleted();


--
-- Name: post post_aggregates_post; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER post_aggregates_post AFTER INSERT OR DELETE ON public.post FOR EACH ROW EXECUTE FUNCTION public.post_aggregates_post();


--
-- Name: post_like post_aggregates_score; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER post_aggregates_score AFTER INSERT OR DELETE ON public.post_like FOR EACH ROW EXECUTE FUNCTION public.post_aggregates_score();


--
-- Name: post post_aggregates_stickied; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER post_aggregates_stickied AFTER UPDATE ON public.post FOR EACH ROW WHEN ((old.stickied IS DISTINCT FROM new.stickied)) EXECUTE FUNCTION public.post_aggregates_stickied();


--
-- Name: comment site_aggregates_comment_delete; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_comment_delete AFTER DELETE ON public.comment FOR EACH ROW WHEN ((old.local = true)) EXECUTE FUNCTION public.site_aggregates_comment_delete();


--
-- Name: comment site_aggregates_comment_insert; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_comment_insert AFTER INSERT ON public.comment FOR EACH ROW WHEN ((new.local = true)) EXECUTE FUNCTION public.site_aggregates_comment_insert();


--
-- Name: community site_aggregates_community_delete; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_community_delete AFTER DELETE ON public.community FOR EACH ROW WHEN ((old.local = true)) EXECUTE FUNCTION public.site_aggregates_community_delete();


--
-- Name: community site_aggregates_community_insert; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_community_insert AFTER INSERT ON public.community FOR EACH ROW WHEN ((new.local = true)) EXECUTE FUNCTION public.site_aggregates_community_insert();


--
-- Name: person site_aggregates_person_delete; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_person_delete AFTER DELETE ON public.person FOR EACH ROW WHEN ((old.local = true)) EXECUTE FUNCTION public.site_aggregates_person_delete();


--
-- Name: person site_aggregates_person_insert; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_person_insert AFTER INSERT ON public.person FOR EACH ROW WHEN ((new.local = true)) EXECUTE FUNCTION public.site_aggregates_person_insert();


--
-- Name: post site_aggregates_post_delete; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_post_delete AFTER DELETE ON public.post FOR EACH ROW WHEN ((old.local = true)) EXECUTE FUNCTION public.site_aggregates_post_delete();


--
-- Name: post site_aggregates_post_insert; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_post_insert AFTER INSERT ON public.post FOR EACH ROW WHEN ((new.local = true)) EXECUTE FUNCTION public.site_aggregates_post_insert();


--
-- Name: site site_aggregates_site; Type: TRIGGER; Schema: public; Owner: owenyoung
--

CREATE TRIGGER site_aggregates_site AFTER INSERT OR DELETE ON public.site FOR EACH ROW EXECUTE FUNCTION public.site_aggregates_site();


--
-- Name: comment_aggregates comment_aggregates_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_aggregates
    ADD CONSTRAINT comment_aggregates_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment comment_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment
    ADD CONSTRAINT comment_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_like comment_like_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like
    ADD CONSTRAINT comment_like_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_like comment_like_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like
    ADD CONSTRAINT comment_like_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_like comment_like_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_like
    ADD CONSTRAINT comment_like_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment comment_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment
    ADD CONSTRAINT comment_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment comment_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment
    ADD CONSTRAINT comment_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_report comment_report_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report
    ADD CONSTRAINT comment_report_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_report comment_report_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report
    ADD CONSTRAINT comment_report_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_report comment_report_resolver_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_report
    ADD CONSTRAINT comment_report_resolver_id_fkey FOREIGN KEY (resolver_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_saved comment_saved_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_saved
    ADD CONSTRAINT comment_saved_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: comment_saved comment_saved_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.comment_saved
    ADD CONSTRAINT comment_saved_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_aggregates community_aggregates_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_aggregates
    ADD CONSTRAINT community_aggregates_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_block community_block_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_block
    ADD CONSTRAINT community_block_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_block community_block_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_block
    ADD CONSTRAINT community_block_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_follower community_follower_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_follower
    ADD CONSTRAINT community_follower_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_follower community_follower_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_follower
    ADD CONSTRAINT community_follower_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_moderator community_moderator_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_moderator
    ADD CONSTRAINT community_moderator_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_moderator community_moderator_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_moderator
    ADD CONSTRAINT community_moderator_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_person_ban community_person_ban_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_person_ban
    ADD CONSTRAINT community_person_ban_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: community_person_ban community_person_ban_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.community_person_ban
    ADD CONSTRAINT community_person_ban_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: local_user local_user_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.local_user
    ADD CONSTRAINT local_user_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_add_community mod_add_community_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add_community
    ADD CONSTRAINT mod_add_community_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_add_community mod_add_community_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add_community
    ADD CONSTRAINT mod_add_community_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_add_community mod_add_community_other_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add_community
    ADD CONSTRAINT mod_add_community_other_person_id_fkey FOREIGN KEY (other_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_add mod_add_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add
    ADD CONSTRAINT mod_add_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_add mod_add_other_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_add
    ADD CONSTRAINT mod_add_other_person_id_fkey FOREIGN KEY (other_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_ban_from_community mod_ban_from_community_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban_from_community
    ADD CONSTRAINT mod_ban_from_community_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_ban_from_community mod_ban_from_community_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban_from_community
    ADD CONSTRAINT mod_ban_from_community_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_ban_from_community mod_ban_from_community_other_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban_from_community
    ADD CONSTRAINT mod_ban_from_community_other_person_id_fkey FOREIGN KEY (other_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_ban mod_ban_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban
    ADD CONSTRAINT mod_ban_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_ban mod_ban_other_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_ban
    ADD CONSTRAINT mod_ban_other_person_id_fkey FOREIGN KEY (other_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_lock_post mod_lock_post_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_lock_post
    ADD CONSTRAINT mod_lock_post_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_lock_post mod_lock_post_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_lock_post
    ADD CONSTRAINT mod_lock_post_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_comment mod_remove_comment_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_comment
    ADD CONSTRAINT mod_remove_comment_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_comment mod_remove_comment_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_comment
    ADD CONSTRAINT mod_remove_comment_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_community mod_remove_community_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_community
    ADD CONSTRAINT mod_remove_community_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_community mod_remove_community_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_community
    ADD CONSTRAINT mod_remove_community_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_post mod_remove_post_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_post
    ADD CONSTRAINT mod_remove_post_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_remove_post mod_remove_post_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_remove_post
    ADD CONSTRAINT mod_remove_post_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_sticky_post mod_sticky_post_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_sticky_post
    ADD CONSTRAINT mod_sticky_post_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_sticky_post mod_sticky_post_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_sticky_post
    ADD CONSTRAINT mod_sticky_post_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_transfer_community mod_transfer_community_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_transfer_community
    ADD CONSTRAINT mod_transfer_community_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_transfer_community mod_transfer_community_mod_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_transfer_community
    ADD CONSTRAINT mod_transfer_community_mod_person_id_fkey FOREIGN KEY (mod_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: mod_transfer_community mod_transfer_community_other_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.mod_transfer_community
    ADD CONSTRAINT mod_transfer_community_other_person_id_fkey FOREIGN KEY (other_person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: password_reset_request password_reset_request_local_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.password_reset_request
    ADD CONSTRAINT password_reset_request_local_user_id_fkey FOREIGN KEY (local_user_id) REFERENCES public.local_user(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_aggregates person_aggregates_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_aggregates
    ADD CONSTRAINT person_aggregates_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_ban person_ban_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_ban
    ADD CONSTRAINT person_ban_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_block person_block_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_block
    ADD CONSTRAINT person_block_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_block person_block_target_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_block
    ADD CONSTRAINT person_block_target_id_fkey FOREIGN KEY (target_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_mention person_mention_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_mention
    ADD CONSTRAINT person_mention_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.comment(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: person_mention person_mention_recipient_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.person_mention
    ADD CONSTRAINT person_mention_recipient_id_fkey FOREIGN KEY (recipient_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_aggregates post_aggregates_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_aggregates
    ADD CONSTRAINT post_aggregates_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post post_community_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_community_id_fkey FOREIGN KEY (community_id) REFERENCES public.community(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post post_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_like post_like_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT post_like_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_like post_like_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT post_like_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_read post_read_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_read
    ADD CONSTRAINT post_read_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_read post_read_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_read
    ADD CONSTRAINT post_read_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_report post_report_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report
    ADD CONSTRAINT post_report_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_report post_report_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report
    ADD CONSTRAINT post_report_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_report post_report_resolver_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_report
    ADD CONSTRAINT post_report_resolver_id_fkey FOREIGN KEY (resolver_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_saved post_saved_person_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_saved
    ADD CONSTRAINT post_saved_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_saved post_saved_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.post_saved
    ADD CONSTRAINT post_saved_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: private_message private_message_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.private_message
    ADD CONSTRAINT private_message_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: private_message private_message_recipient_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.private_message
    ADD CONSTRAINT private_message_recipient_id_fkey FOREIGN KEY (recipient_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: site_aggregates site_aggregates_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site_aggregates
    ADD CONSTRAINT site_aggregates_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.site(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: site site_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owenyoung
--

ALTER TABLE ONLY public.site
    ADD CONSTRAINT site_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.person(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

