CREATE UNIQUE INDEX unique_index_post_favorites ON public.post_favorites USING btree (account_id, post_id);