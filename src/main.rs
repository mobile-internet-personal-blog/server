use axum::Router;
use log;
use rusite::{api::api_route, db::connection::Database, error::Error, utils::AppState};

async fn initialize() -> Result<AppState, Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db = Database::new().await;
    AppState::new(db).await
}

#[tokio::main]
async fn main () -> Result<(), Error> {
    
    let state = initialize().await?;

    let app = Router::new()
        .nest("/api", api_route(state));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8216").await.unwrap();

        log::info!("LISTENING on {:?}\n", listener.local_addr());

        axum::serve(listener, app).await.unwrap();
    
    Ok(())
}