use std::env;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Encode;

use crate::{error::Error, utils::AppState};
use reqwest::{self, header};

#[derive(Deserialize)]
pub struct LoginParams {
    code : String,
    third_party_provider: ThirdPartyProvider,
}

#[derive(Serialize)]
pub struct User {
    id: String,
    name: String,
    avatar_url: String,
    third_party_provider: ThirdPartyProvider,
}

impl User {
    fn new(
        id: &str,
        name: &str,
        avatar_url: &str,
        third_party_provider: ThirdPartyProvider
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            avatar_url: avatar_url.to_string(),
            third_party_provider,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthBody {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize, Encode)]
pub enum ThirdPartyProvider {
    Github,
}

#[derive(Serialize)]
pub struct RequestTokenParams {
    client_id: String,
    client_secret: String,
    code: String,
}

impl RequestTokenParams {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        code: &str,
    ) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            code: code.to_string()
        }
    }
}

pub async fn handler_login (
    State(state) : State<AppState>,
    parmas: Json<LoginParams>,
) -> Result<Json<User>, Error> {
    println!("->> {:<12} - handler_login", "HANDLER");
    match parmas.third_party_provider {
        ThirdPartyProvider::Github => {
            let request_token_params = RequestTokenParams::new (
                &env::var("GITHUB_CLIENTID").expect("Github client id not found"), 
                &env::var("GITHUB_CLIENTSECRET").expect("Github client secret not found"),
                &parmas.code 
            );
            let oauthtoken = request_token(&request_token_params).await?;
            let user = request_user_info(&oauthtoken, ThirdPartyProvider::Github).await?;
            let res = state.db.query_uid(&user.id, ThirdPartyProvider::Github).await;
            match res {
                Ok(uid) => {
                    state.db.update_user(&uid, &oauthtoken.access_token).await?;
                }
                Err(Error::DbError(sqlx::Error::RowNotFound)) => {
                    state.db.create_user(&user.id, ThirdPartyProvider::Github, &oauthtoken.access_token).await?;
                }
                Err(e) => return Err(e),
            }
            Ok(Json(user))
        }
    }
}

async fn request_token (
    params: &RequestTokenParams
) -> Result<OAuthBody, Error> {
    let res = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .form(params)
        .header("accept", "application/json")
        .send()
        .await?;
    let oauthtoken: OAuthBody = serde_json::from_str(&res.text().await?)?;

    Ok(oauthtoken)
}

async fn request_user_info (
    params: &OAuthBody,
    third_party_provider: ThirdPartyProvider,
) -> Result<User, Error> {
    let res = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header(header::USER_AGENT, "h")
        .header("accept", "application/json")
        .header("Authorization", format!("token {}", params.access_token))
        .send()
        .await?;
    let user = res.text().await?;
    let user : Value = serde_json::from_str(&user)?;
    let id = user["id"].to_string();
    let name = user["name"].to_string();
    let avatar_url = user["avatar_url"].to_string();

    Ok(User::new(&id, &name, &avatar_url, third_party_provider))
}