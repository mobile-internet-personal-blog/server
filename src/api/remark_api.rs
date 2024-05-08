use axum::{extract::{Query, State}, Json};
use chrono::Utc;
use serde::Deserialize;

use crate::{error::{ApiError, Error, Result}, model::Message, utils::AppState, Uuid};

use super::login_api::ThirdPartyProvider;

#[derive(Deserialize)]
pub struct EidParams {
    eid: Uuid,
}

pub async fn handler_remark_list (
    State(state): State<AppState>,
    Query(params): Query<EidParams>,
) -> Result<Json<Vec<Message>>> {
    println!("->> {:<12} - handler_remark_list", "HANDLER");
    match state.remarks.get(&params.eid) {
        None => Err(Error::from(ApiError::NotFound)),
        Some(remark_list) => Ok(Json(remark_list.get_vec().await?))
    }
}

pub async fn handler_chat_list (
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>> {
    println!("->> {:<12} - handler_chat_list", "HANDLER");
    Ok(Json(state.chatmsg.get_vec().await?))
}

#[derive(Deserialize)]
pub struct RemarkParams {
    eid: Uuid,
    open_id: String,
    third_party_provider: ThirdPartyProvider,
    content: String,
}

#[derive(Deserialize)]
pub struct ChatParams {
    open_id: String,
    third_party_provider: ThirdPartyProvider,
    content: String,
}

pub async fn handler_send_remark (
    State(state): State<AppState>,
    params: Json<RemarkParams>,
) -> Result<()> {
    let uid = state.db.query_uid(&params.open_id, &params.third_party_provider).await?;
    let curent = Utc::now();
    match state.db.create_remark(&params.eid, &uid, &params.content).await {
        Ok(_) => {
            state.remarks.get(&params.eid).unwrap().create_msg(Message::new(
                &uid, &params.content, curent)).await?
        }
        Err(e) => {
           return Err(e); 
        }
    }
    Ok(())
}

pub async fn handler_send_chatmsg (
    State(state): State<AppState>,
    params: Json<ChatParams>
) -> Result<()> {
    let uid = state.db.query_uid(&params.open_id, &params.third_party_provider).await?;
    let curent = Utc::now();
    match state.db.create_message(&uid, &params.content).await {
        Ok(_) => {
            state.chatmsg.create_msg(Message::new(
                &uid, &params.content, curent)).await?
        }
        Err(e) => {
           return Err(e); 
        }
    }
    Ok(())
}