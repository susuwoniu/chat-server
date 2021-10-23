-- Add migration script here
CREATE EXTENSION IF NOT EXISTS postgis;
-- create enum type identity_type
CREATE TYPE identity_type AS ENUM ('phone','email','wechat', 'weibo','apple','google','facebook','twitter');
CREATE TYPE gender AS ENUM ('male', 'female','intersex','other','unknown');
CREATE TYPE target_gender AS ENUM ('male', 'female','all');

-- Your SQL goes here
-- 用户账号表
CREATE TABLE accounts (
   id bigint NOT NULL PRIMARY KEY,
   name character varying(255) NOT NULL,
   bio character varying(255) DEFAULT ''::text NOT NULL,
   gender gender DEFAULT 'unknown' NOT NULL,
   admin boolean DEFAULT false NOT NULL,
   moderator boolean DEFAULT false NOT NULL,
   vip boolean Default false NOT NULL,
   posts_count bigint DEFAULT 0  NOT NULL,
   likes_count bigint DEFAULT 0  NOT NULL,
   show_age boolean DEFAULT true NOT NULL,
   show_distance boolean DEFAULT true NOT NULL,
   deleted boolean DEFAULT false NOT NULL,
   deleted_at timestamp,
   suspended boolean DEFAULT false NOT NULL,
   suspended_at timestamp,
   suspended_until timestamp,
   suspended_reason text,
   birthday date,
   phone_country_code integer,
   phone_number character varying(255),
   location character varying(255),
   country_id integer,
   state_id integer,
   city_id integer,
   avatar character varying,
   profile_images json,
   avatar_updated_at timestamp without time zone, 
   created_at timestamp without time zone DEFAULT now() NOT NULL,
   updated_at timestamp without time zone NOT NULL,
   approved boolean DEFAULT false NOT NULL,
   approved_at timestamp,
   invite_id bigint
);

--用户登录表
CREATE TABLE account_auths (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    signin_count bigint DEFAULT 0 NOT NULL,
    -- 10: mobile, 20: wechat, 30: apple
    identity_type identity_type NOT NULL,
    identifier character varying(1024),

    hash BYTEA,
    salt VARCHAR(255),
    third_party_data json,
    third_party_token_expires_at timestamp without time zone,
    third_party_refresh_token_expires_at timestamp without time zone,
    current_signin_at timestamp without time zone,
    last_signin_at timestamp,
    current_signin_ip inet,
    last_signin_ip inet,
    account_id bigint NOT NULL,
    signup_ip inet,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at boolean DEFAULT false NOT NULL
);

CREATE UNIQUE INDEX index_identity_type_and_identifier ON account_auths USING btree (identifier,identity_type,deleted);

-- 用户登录记录表

CREATE TABLE login_activities (
    id bigint NOT NULL PRIMARY KEY,
    account_auth_id bigint NOT NULL,
    account_id bigint NOT NULL,
    -- iOS: 10, android: 20, web: 30
    client_id bigint NOT NULL,
    success boolean NOT NULL,
    failure_reason character varying,
    ip inet,
    user_agent character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp
);

CREATE INDEX index_login_activities_account_id ON account_auths USING btree (account_id,deleted);

-- 邀请表
CREATE TABLE invites (
    id bigint NOT NULL PRIMARY KEY,
    account_id bigint NOT NULL,
    code character varying NOT NULL UNIQUE,
    max_uses bigint,
    uses bigint DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE INDEX index_invites_code ON invites USING btree (code);
CREATE INDEX index_invites_account_id ON invites USING btree (account_id);

-- blocks表
-- 限制一个用户可以blocks的人

CREATE TABLE blocks (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp
);

CREATE INDEX index_blocks_on_account_id_and_target_account_id ON blocks USING btree (account_id, target_account_id,deleted);
CREATE INDEX index_blocks_on_target_account_id ON blocks USING btree (target_account_id,deleted);
CREATE INDEX index_blocks_on_account_id ON blocks USING btree (account_id,deleted);

-- likes 表
CREATE TABLE likes (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL,
    deleted boolean DEFAULT false NOT NULL
);

CREATE UNIQUE INDEX index_likes_on_account_id_and_target_account_id ON likes USING btree (account_id, target_account_id);

CREATE INDEX index_likes_on_target_account_id ON likes USING btree (target_account_id);
CREATE INDEX index_likes_on_account_id ON likes USING btree (account_id);


-- reports

CREATE TABLE reports (
    id bigint NOT NULL PRIMARY KEY,
    report_type character varying(255) NOT NULL,
    content text DEFAULT ''::text NOT NULL,
    images json,
    action_taken boolean DEFAULT false NOT NULL,
    action_taken_by_account_id bigint,
    action_comment text  DEFAULT ''::text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL
);

-- post_templates 表
CREATE TABLE post_templates (
    id bigint NOT NULL PRIMARY KEY,
    content text DEFAULT ''::text NOT NULL,
    used_count bigint DEFAULT 0 NOT NULL,
    skip_count bigint DEFAULT 0 NOT NULL,
    background_image character varying,
    background_color character varying(255),
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    verified boolean DEFAULT false NOT NULL
);


-- posts 表

CREATE TABLE posts (
    id bigint NOT NULL PRIMARY KEY,
    content text DEFAULT ''::text NOT NULL,
    background_image character varying,
    background_color character varying(255),
    post_template_id bigint NOT NULL,
    client_id bigint NOT NULL,
    skip_count bigint DEFAULT 0 NOT NULL,
    view_count bigint DEFAULT 0 NOT NULL,
    target_gender target_gender DEFAULT 'all' NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    time_cursor timestamp without time zone NOT NULL,
    sensitive boolean DEFAULT false NOT NULL,
    account_id bigint NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    ip inet
);
CREATE INDEX index_posts_on_account_id ON posts USING btree (account_id);

-- views 表

CREATE TABLE views (
    id bigint NOT NULL PRIMARY KEY,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    post_id bigint NOT NULL
);
CREATE INDEX index_views_on_post_id ON views USING btree (post_id);