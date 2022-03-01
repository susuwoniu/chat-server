use crate::{
    account::model::Account,
    alias::Pool,
    global::Config,
    middleware::{Auth, Locale, Qs},
    post::{
        model::{ApiPostFilter, Post, PostFilter},
        service::get_post::get_posts,
    },
    types::Image,
    util::color::number_to_hex,
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use ramhorns::{Content, Template};
use std::fs::File;
use std::io::prelude::*;
use substring::Substring;
#[derive(Content)]
struct PostData {
    pub id: i64,
    pub description: String,
    pub content: String,
    pub viewed_count: i64,
    pub favorite_count: i64,
    pub background_color: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
    pub author_name: String,
    pub author_avatar: Option<Image>,
    pub site_name: String,
    pub title: String,
    pub style: String,
}
pub async fn get_post_page_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Qs(filter): Qs<ApiPostFilter>,
    option_auth: Option<Auth>,
) -> impl IntoResponse {
    let post_filter_result = PostFilter::try_from(filter);
    if post_filter_result.is_err() {
        return Err((StatusCode::BAD_REQUEST, format!("post filter invalid")).into_response());
    }
    let posts_filter = post_filter_result.unwrap();
    let data_result = get_posts(&locale, &pool, posts_filter, option_auth, false, false).await;
    if data_result.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            data_result.err().unwrap(),
        )
            .into_response());
    }

    let data = data_result.unwrap();
    if data.data.len() > 0 {
        // Standard Mustache action here
        #[cfg(not(debug_assertions))]
        let source = include_str!("../../../../resources/templates/post.html");
        #[cfg(not(debug_assertions))]
        let css_source = include_str!("../../../../dist/styles/main.css").to_string();
        // get file content
        #[cfg(debug_assertions)]
        let mut file =
            File::open("resources/templates/post.html").expect("Unable to open the file");
        #[cfg(debug_assertions)]
        let mut source = String::new();
        #[cfg(debug_assertions)]
        file.read_to_string(&mut source)
            .expect("Unable to read the file");

        // get css content
        #[cfg(debug_assertions)]
        let mut css_file = File::open("dist/styles/main.css").expect("Unable to open the file");
        #[cfg(debug_assertions)]
        let mut css_source = String::new();
        #[cfg(debug_assertions)]
        css_file
            .read_to_string(&mut css_source)
            .expect("Unable to read the file");
        let tpl = Template::new(source).unwrap();
        let Post {
            id,
            content,
            viewed_count,
            favorite_count,
            background_color,
            color,
            author,
            created_at,
            updated_at,
            ..
        } = data.data[0].clone();
        let Account { name, avatar, .. } = author;
        let content_without_new_line = str::replace(&content, "\n", "");
        let title = content_without_new_line.substring(0, 36).to_string();

        let description = content_without_new_line.substring(0, 140).to_string();
        let cfg = Config::global();
        let rendered = tpl.render(&PostData {
            id,
            title: title,
            description: description,
            site_name: cfg.site.name.clone(),
            content,
            viewed_count,
            favorite_count,
            background_color: number_to_hex(background_color),
            color: number_to_hex(color),
            author_name: name,
            author_avatar: avatar,
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            style: css_source,
        });
        Ok(Html(rendered).into_response())
    } else {
        Err((StatusCode::BAD_REQUEST, format!("Can not found the post")).into_response())
    }
}
