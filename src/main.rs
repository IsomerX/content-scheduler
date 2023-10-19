use axum::{
    routing::{get, post},
    Router,
};
use sqlx::query;
use sqlx::Row;
use std::error::Error;
use tokio;

mod postgres;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = postgres::create_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/healthcheck", get(routes::healthcheck))
        .route("/create", post(routes::create))
        .route("/get", get(routes::get));

    let addr = "0.0.0.0:3000".parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}