--
-- Name: favorites; Type: TABLE; Schema: public; Owner: -
--

--
-- Name: post_favorites; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_favorites (
    id bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_id bigint NOT NULL,
    post_id bigint NOT NULL,
    post_account_id bigint NOT NULL
);

CREATE INDEX index_post_favorites ON public.post_favorites USING btree (account_id);
CREATE INDEX index_post_favorites_post_id ON public.post_favorites USING btree (post_id);
