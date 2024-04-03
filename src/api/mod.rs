pub mod login_api;
mod regular_api;

use axum::{routing::{get, post}, Router};

use crate::utils::AppState;

use self::{login_api::handler_login, regular_api::{handler_basic_info, handler_essay_content, handler_essay_info_list}};


pub fn api_route(state: AppState) -> Router {
    Router::new()
        .route("/basicinfo", get(handler_basic_info))
        .route("/essayinfolist", get(handler_essay_info_list))
        .route("/queryessaycontent", get(handler_essay_content))
        .route("/login", post(handler_login))
        .with_state(state)
}