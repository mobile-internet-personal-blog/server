mod login_api;
mod blog_regular_api;

use axum::{routing::get, Router};

use crate::utils::AppState;

use self::blog_regular_api::{handler_blog_info_list, handler_essay_content};

pub fn api_route(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async {"hello world!"}))
        .route("/essayinfolist", get(handler_blog_info_list))
        .route("/essaycontent", get(handler_essay_content))
        .with_state(state)
}