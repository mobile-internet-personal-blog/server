use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::{config::Config, error::{ApiError, Error}, model::EssayInfo, utils::AppState, Uuid};

pub async fn handler_basic_info (
    State(state): State<AppState>,
) -> Result<Json<Config>, Error> {
    println!("->> {:<12} - handler_basic_info", "HANDLER");
    Ok(Json(state.config))
} 

pub async fn handler_essay_info_list (
    State(state): State<AppState>,
) -> Result<Json<Vec<EssayInfo>>, Error> {
    println!("->> {:<12} - handler_essay_info_list", "HANDLER");
    Ok(Json(state.essaylist.get_list().await?))
}

#[derive(Deserialize)]
pub struct EidParams {
    eid: Uuid,
}

pub async fn handler_essay_content (
    Query(params): Query<EidParams>,
    State(state): State<AppState>,
) -> Result<String, ApiError> {
    println!("->> {:<12} - handler_essay_content", "HANDLER");
    match state.essaymap.get(&params.eid) {
        Some(content) => Ok(content.to_string()),
        None => Err(ApiError::NotFound),
    }
}