use axum::{routing::get, Router};
use log;
use reqwest::Method;
use rusite::{api::api_route, error::Error, utils::AppState};
use tower_http::cors::{any, CorsLayer};

async fn initialize() -> Result<AppState, Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    AppState::new().await
}

#[tokio::main]
async fn main () -> Result<(), Error> {
    let state = initialize().await?;

    let app = Router::new()
        .route("/", get(|| async {"running..."}))
        .nest("/api", api_route(state))
        .layer(CorsLayer::new()
                    .allow_methods(vec![Method::GET, Method::POST])
                    .allow_origin(any()));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8216").await.unwrap();
        log::info!("LISTENING on {:?}\n", listener.local_addr());

        axum::serve(listener, app).await.unwrap();
    
    Ok(())
}