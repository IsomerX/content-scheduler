use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::query;
use sqlx::Row;
use std::error::Error;
use tokio;
use std::net::{ToSocketAddrs};

mod postgres;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = postgres::create_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/healthcheck", get(routes::healthcheck))
        .route("/create", post(routes::create))
        .route("/get", get(routes::get))
        .route("/update", put(routes::update));

    let mut addr = "172.20.0.3:3000".to_socket_addrs().unwrap();
    let socket_addr = match addr.next() {
        Some(socket_addr) => socket_addr.clone(),
        None => {
            eprintln!("Failed to resolve socket address");
            return Ok(());
        }
    };
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
