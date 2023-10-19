use std::error::Error;

use axum::{http::StatusCode, Json};
use chrono::DateTime;
use chrono::Utc;
use sqlx::query;
use sqlx::Row;
use sqlx::{pool, PgPool};

use crate::routes;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let url = "postgres://dbuser:mysecretpassword@db:5432/content";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    Ok(pool)
}

pub async fn get_entries() -> Result<Json<Vec<routes::Content>>, Box<dyn Error>> {
    let pool = create_pool().await.unwrap();
    let query = "SELECT * FROM content";

    let mut entries = Vec::new();

    let recs = sqlx::query(query).fetch_all(&pool).await.unwrap();

    for rec in recs {
        let date: DateTime<Utc> = rec.get(1);
        let entry = routes::Content {
            id: rec.get(0),
            date: date,
            content: rec.get(2),
            linkedin: rec.get(3),
            twitter: rec.get(4),
            current: rec.get(5),
        };

        entries.push(entry);
    }

    Ok(Json(entries))
}

pub async fn create_entry(Json(entry): Json<routes::Content>) -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    let query = "INSERT INTO content (id, date, content, linkedin, twitter, current) VALUES ($1, $2, $3, $4, $5, $6)";

    sqlx::query(query)
        .bind(entry.id)
        .bind(entry.date)
        .bind(entry.content)
        .bind(entry.linkedin)
        .bind(entry.twitter)
        .bind(entry.current)
        .execute(&pool)
        .await
        .unwrap();

    Ok(())
}
