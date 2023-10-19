use crate::postgres;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid;

pub async fn healthcheck() -> &'static str {
    "live"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateEntry {
    content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
    pub id: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub content: String,
    pub linkedin: String,
    pub twitter: String,
    pub current: bool,
}

pub async fn get() -> (StatusCode, Json<Vec<Content>>) {
    let entries = postgres::get_entries().await.unwrap();
    (StatusCode::OK, entries)
}

#[axum_macros::debug_handler]
pub async fn create(Json(payload): Json<CreateEntry>) -> (StatusCode, Json<Content>) {
    let id = uuid::Uuid::new_v4().to_string();
    let content = Content {
        id,
        date: chrono::Utc::now(),
        content: payload.content,
        linkedin: "".to_string(),
        twitter: "".to_string(),
        current: false,
    };

    postgres::create_entry(Json(content.clone())).await.unwrap();

    (StatusCode::CREATED, Json(content))
}
