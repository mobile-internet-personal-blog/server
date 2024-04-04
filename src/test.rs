use std::env;

use crate::{api::login_api::{OAuthBody, RequestTokenParams}, config::Config};

#[test]
fn get_basicinfo() {
    let conf = Config::default();
    println!("{}", serde_json::to_string_pretty(&conf).unwrap());
}

#[tokio::test]
async fn get_token() {
    dotenv::dotenv().ok();
}

