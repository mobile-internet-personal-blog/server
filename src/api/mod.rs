pub mod login_api;
mod regular_api;
mod remark_api;

use axum::{routing::{get, post}, Router};

use crate::utils::AppState;

use self::{login_api::handler_login, regular_api::{handler_basic_info, handler_essay_content, handler_essay_info_list}, remark_api::{handler_chat_list, handler_remark_list, handler_send_chatmsg, handler_send_remark}};


pub fn api_route(state: AppState) -> Router {
    Router::new()
        .route("/basicinfo", get(handler_basic_info))
        .route("/essayinfolist", get(handler_essay_info_list))
        .route("/queryessaycontent", get(handler_essay_content))
        .route("/login", post(handler_login))
        .route("/remarklist", get(handler_remark_list))
        .route("/chatmsglist", get(handler_chat_list))
        .route("/createremark", post(handler_send_remark))
        .route("/createchatmsg", post(handler_send_chatmsg))
        .with_state(state)
}