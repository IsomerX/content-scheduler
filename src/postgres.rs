use std::error::Error;

use axum::{http::StatusCode, Json};
use chrono::DateTime;
use chrono::Utc;
use sqlx::query;
use sqlx::Row;
use sqlx::{pool, PgPool};

use crate::routes;
use crate::routes::Content;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5342/content";
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
        };

        entries.push(entry);
    }

    Ok(Json(entries))
}

pub async fn create_entry(Json(entry): Json<routes::Content>) -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    let query =
        "INSERT INTO content (id, date, content, linkedin, twitter) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(entry.id)
        .bind(entry.date)
        .bind(entry.content)
        .bind(entry.linkedin)
        .bind(entry.twitter)
        .execute(&pool)
        .await
        .unwrap();

    Ok(())
}

pub async fn get_last_entry() -> Result<Json<Content>, sqlx::Error> {
    let pool = create_pool().await?;
    let query = "SELECT * FROM content ORDER BY date DESC LIMIT 1";

    let rec = sqlx::query(query).fetch_one(&pool).await?;

    let date: DateTime<Utc> = rec.get(1);
    let entry = Content {
        id: rec.get(0),
        date,
        content: rec.get(2),
        linkedin: rec.get(3),
        twitter: rec.get(4),
    };

    Ok(Json(entry))
}

pub async fn update_entry(id: String, content: String) -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    let query = "UPDATE content SET content = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(content)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(())
}
