




--
-- Name: account_auths; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.account_auths (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    signin_count bigint DEFAULT 0 NOT NULL,
    identity_type smallint NOT NULL,
    identifier character varying(1024),
    hash bytea,
    salt character varying(255),
    third_party_data json,
    third_party_token_expires_at timestamp without time zone,
    third_party_refresh_token_expires_at timestamp without time zone,
    current_signin_at timestamp without time zone,
    last_signin_at timestamp without time zone,
    current_signin_ip inet,
    last_signin_ip inet,
    account_id bigint NOT NULL,
    signup_ip inet,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone
);


--
-- Name: account_images; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.account_images (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    _order smallint NOT NULL,
    url character varying NOT NULL,
    height double precision NOT NULL,
    width double precision NOT NULL,
    size bigint NOT NULL,
    mime_type character varying(255) NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone
);


--
-- Name: account_view_records; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.account_view_records (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    viewed_by bigint NOT NULL,
    target_account_id bigint NOT NULL
);


--
-- Name: account_views; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.account_views (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    viewed_by bigint NOT NULL,
    target_account_id bigint NOT NULL,
    viewed_count bigint DEFAULT 0 NOT NULL
);


--
-- Name: accounts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.accounts (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    username character varying(255),
    bio character varying(255) DEFAULT ''::text NOT NULL,
    gender smallint DEFAULT 0 NOT NULL,
    admin boolean DEFAULT false NOT NULL,
    moderator boolean DEFAULT false NOT NULL,
    vip boolean DEFAULT false NOT NULL,
    post_count bigint DEFAULT 0 NOT NULL,
    like_count bigint DEFAULT 0 NOT NULL,
    show_age boolean DEFAULT true NOT NULL,
    show_distance boolean DEFAULT true NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    suspended boolean DEFAULT false NOT NULL,
    suspended_at timestamp without time zone,
    suspended_until timestamp without time zone,
    suspended_reason text,
    birthday date,
    timezone_in_seconds integer,
    phone_country_code integer,
    phone_number character varying(255),
    location character varying(255),
    country_id integer,
    state_id integer,
    city_id integer,
    avatar character varying,
    avatar_updated_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    approved boolean DEFAULT false NOT NULL,
    approved_at timestamp without time zone,
    invite_id bigint,
    name_change_count integer DEFAULT 0 NOT NULL,
    bio_change_count integer DEFAULT 0 NOT NULL,
    gender_change_count integer DEFAULT 0 NOT NULL,
    birthday_change_count integer DEFAULT 0 NOT NULL,
    phone_change_count integer DEFAULT 0 NOT NULL,
    skip_optional_info boolean DEFAULT false NOT NULL,
    profile_image_change_count integer DEFAULT 0 NOT NULL,
    post_template_count bigint DEFAULT 0 NOT NULL,
    show_viewed_action boolean DEFAULT true NOT NULL,
    profile_images json,
    last_post_created_at timestamp without time zone
);


--
-- Name: blocks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.blocks (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL
);


--
-- Name: invites; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.invites (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    code character varying NOT NULL,
    max_uses bigint,
    uses bigint DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: likes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.likes (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    target_account_id bigint NOT NULL
);


--
-- Name: login_activities; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.login_activities (
    id bigint NOT NULL,
    account_auth_id bigint NOT NULL,
    account_id bigint NOT NULL,
    client_id bigint NOT NULL,
    success boolean NOT NULL,
    failure_reason character varying,
    ip inet,
    user_agent character varying,
    device_id character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    client_platform smallint DEFAULT 1 NOT NULL
);


--
-- Name: notification_inboxes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.notification_inboxes (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    unread_count bigint DEFAULT 0 NOT NULL,
    _type smallint NOT NULL,
    last_notification_id bigint NOT NULL,
    last_notification_updated_at timestamp without time zone NOT NULL,
    last_notification_from bigint,
    is_primary boolean DEFAULT false NOT NULL,
    total_count bigint DEFAULT 0 NOT NULL
);


--
-- Name: notifications; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.notifications (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    _type smallint NOT NULL,
    content text DEFAULT ''::text NOT NULL,
    _action smallint NOT NULL,
    from_account_id bigint,
    action_data json,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    is_primary boolean DEFAULT false NOT NULL
);


--
-- Name: post_reply; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_reply (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    replied_by bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);


--
-- Name: post_skip; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_skip (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    skipped_by bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);


--
-- Name: post_templates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_templates (
    id bigint NOT NULL,
    used_count bigint DEFAULT 0 NOT NULL,
    skipped_count bigint DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    featured boolean DEFAULT false NOT NULL,
    featured_at timestamp without time zone,
    featured_by bigint,
    deleted boolean DEFAULT false NOT NULL,
    sensitive boolean DEFAULT false NOT NULL,
    ip inet,
    deleted_at timestamp without time zone,
    deleted_by bigint,
    time_cursor bigint DEFAULT 0 NOT NULL,
    priority bigint,
    title character varying(255) NOT NULL,
    content text
);


--
-- Name: post_view; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_view (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    viewed_by bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);


--
-- Name: posts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.posts (
    id bigint NOT NULL,
    content text DEFAULT ''::text NOT NULL,
    post_template_id bigint NOT NULL,
    client_id bigint NOT NULL,
    skipped_count bigint DEFAULT 0 NOT NULL,
    viewed_count bigint DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    sensitive boolean DEFAULT false NOT NULL,
    account_id bigint NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    ip inet,
    time_cursor bigint NOT NULL,
    approved boolean DEFAULT true NOT NULL,
    approved_at timestamp without time zone,
    approved_by bigint,
    featured boolean DEFAULT false NOT NULL,
    featured_at timestamp without time zone,
    featured_by bigint,
    target_gender smallint,
    visibility smallint DEFAULT 1 NOT NULL,
    gender smallint NOT NULL,
    deleted_by bigint,
    replied_count bigint DEFAULT 0 NOT NULL,
    birthday date,
    background_color bigint NOT NULL,
    color bigint NOT NULL,
    post_template_title character varying(255) NOT NULL,
    geom public.geometry(Point,4326),
    time_cursor_change_count integer DEFAULT 0 NOT NULL
);


--
-- Name: reports; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.reports (
    id bigint NOT NULL,
    content text DEFAULT ''::text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    _type smallint DEFAULT 0 NOT NULL,
    state smallint DEFAULT 0 NOT NULL,
    related_post_id bigint,
    related_account_id bigint,
    replied_content text,
    replied_by bigint,
    replied_at timestamp without time zone,
    images character varying(2048)[] DEFAULT ARRAY[]::character varying[] NOT NULL
);



--
-- Name: account_auths account_auths_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account_auths
    ADD CONSTRAINT account_auths_pkey PRIMARY KEY (id);


--
-- Name: account_images account_images_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account_images
    ADD CONSTRAINT account_images_pkey PRIMARY KEY (id);


--
-- Name: account_view_records account_view_records_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account_view_records
    ADD CONSTRAINT account_view_records_pkey PRIMARY KEY (id);


--
-- Name: account_views account_views_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account_views
    ADD CONSTRAINT account_views_pkey PRIMARY KEY (id);


--
-- Name: accounts accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id);


--
-- Name: blocks blocks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocks
    ADD CONSTRAINT blocks_pkey PRIMARY KEY (id);


--
-- Name: invites invites_code_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_code_key UNIQUE (code);


--
-- Name: invites invites_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_pkey PRIMARY KEY (id);


--
-- Name: likes likes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.likes
    ADD CONSTRAINT likes_pkey PRIMARY KEY (id);


--
-- Name: login_activities login_activities_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.login_activities
    ADD CONSTRAINT login_activities_pkey PRIMARY KEY (id);


--
-- Name: notification_inboxes notification_inboxes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.notification_inboxes
    ADD CONSTRAINT notification_inboxes_pkey PRIMARY KEY (id);


--
-- Name: notifications notifications_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_pkey PRIMARY KEY (id);


--
-- Name: post_reply post_reply_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_reply
    ADD CONSTRAINT post_reply_pkey PRIMARY KEY (id);


--
-- Name: post_skip post_skip_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_skip
    ADD CONSTRAINT post_skip_pkey PRIMARY KEY (id);


--
-- Name: post_templates post_templates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_templates
    ADD CONSTRAINT post_templates_pkey PRIMARY KEY (id);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: reports reports_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reports
    ADD CONSTRAINT reports_pkey PRIMARY KEY (id);


--
-- Name: post_view views_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_view
    ADD CONSTRAINT views_pkey PRIMARY KEY (id);


--
-- Name: index_account_id_images; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_account_id_images ON public.account_images USING btree (account_id);


--
-- Name: index_account_id_type_on_notification_inboxes; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_account_id_type_on_notification_inboxes ON public.notification_inboxes USING btree (account_id);


--
-- Name: index_blocks_on_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_blocks_on_account_id ON public.blocks USING btree (account_id);


--
-- Name: index_blocks_on_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_blocks_on_target_account_id ON public.blocks USING btree (target_account_id);


--
-- Name: index_for_account_id_and_type_of_notifications; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_for_account_id_and_type_of_notifications ON public.notifications USING btree (account_id);


--
-- Name: index_identity_type_and_identifier; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_identity_type_and_identifier ON public.account_auths USING btree (identifier, identity_type);


--
-- Name: index_invites_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_invites_account_id ON public.invites USING btree (account_id);


--
-- Name: index_invites_code; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_invites_code ON public.invites USING btree (code);


--
-- Name: index_likes_on_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_likes_on_account_id ON public.likes USING btree (account_id);


--
-- Name: index_likes_on_account_id_and_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX index_likes_on_account_id_and_target_account_id ON public.likes USING btree (account_id, target_account_id);


--
-- Name: index_likes_on_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_likes_on_target_account_id ON public.likes USING btree (target_account_id);


--
-- Name: index_login_activities_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_login_activities_account_id ON public.account_auths USING btree (account_id);


--
-- Name: index_post_reply_on_post_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_reply_on_post_account_id ON public.post_reply USING btree (post_account_id);


--
-- Name: index_post_reply_on_replied_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_reply_on_replied_by ON public.post_reply USING btree (replied_by);


--
-- Name: index_post_skip_on_post_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_skip_on_post_account_id ON public.post_skip USING btree (post_account_id);


--
-- Name: index_post_skip_on_skipped_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_skip_on_skipped_by ON public.post_skip USING btree (skipped_by);


--
-- Name: index_post_view_on_post_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_view_on_post_account_id ON public.post_view USING btree (post_account_id);


--
-- Name: index_post_view_on_viewed_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_post_view_on_viewed_by ON public.post_view USING btree (viewed_by);


--
-- Name: index_posts_on_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_posts_on_account_id ON public.posts USING btree (account_id);


--
-- Name: index_records_view_on_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_records_view_on_account_id ON public.account_view_records USING btree (viewed_by);


--
-- Name: index_records_view_on_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_records_view_on_target_account_id ON public.account_view_records USING btree (target_account_id);


--
-- Name: index_unique_images; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX index_unique_images ON public.account_images USING btree (account_id, _order, COALESCE(deleted_at, '0001-01-01T00:00:00Z'));


--
-- Name: index_view_on_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_view_on_account_id ON public.account_views USING btree (viewed_by);


--
-- Name: index_view_on_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_view_on_target_account_id ON public.account_views USING btree (target_account_id);


--
-- Name: index_views_on_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_views_on_post_id ON public.post_view USING btree (post_id);


--
-- Name: posts_location_index_geom; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX posts_location_index_geom ON public.posts USING gist (geom);


--
-- Name: unique_index_account_id_type_on_notification_inboxes; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_account_id_type_on_notification_inboxes ON public.notification_inboxes USING btree (account_id, _type);


--
-- Name: unique_index_account_views_on_account_id_and_viewed_by; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_account_views_on_account_id_and_viewed_by ON public.account_views USING btree (viewed_by, target_account_id);


--
-- Name: unique_index_accounts_on_phone_number; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX unique_index_accounts_on_phone_number ON public.accounts USING btree (phone_number);


--
-- Name: unique_index_accounts_on_username; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_accounts_on_username ON public.accounts USING btree (username, COALESCE(deleted_at, '0001-01-01T00:00:00Z'));


--
-- Name: unique_index_blocks_on_account_id_and_target_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_blocks_on_account_id_and_target_account_id ON public.blocks USING btree (account_id, target_account_id);


--
-- Name: unique_index_identity_type_and_identifier; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_identity_type_and_identifier ON public.account_auths USING btree (identifier, identity_type, COALESCE(deleted_at, '0001-01-01T00:00:00Z'));


--
-- Name: unique_index_post_reply_on_post_id_and_replied_by; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_post_reply_on_post_id_and_replied_by ON public.post_reply USING btree (post_id, replied_by);


--
-- Name: unique_index_post_skip_on_post_id_and_skipped_by; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_post_skip_on_post_id_and_skipped_by ON public.post_skip USING btree (post_id, skipped_by);


--
-- Name: unique_index_post_view_on_post_id_and_viewed_by; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX unique_index_post_view_on_post_id_and_viewed_by ON public.post_view USING btree (post_id, viewed_by);


--
-- PostgreSQL database dump complete
--

