use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use sqlx::{prelude::FromRow, Pool, Postgres};

pub struct TodoRepo {
  pool: Pool<Postgres>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum Status {
  PENDING,
  IN_PROGRESS,
  DONE,
  CANCELLED,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
  pub title: String,
  pub description: String,
  pub created_at: Option<DateTime<Utc>>,
  pub last_modified: Option<DateTime<Utc>>,
  pub status: Option<Status>,
}

impl TodoRepo {
  pub fn new(pool: Pool<Postgres>) -> Self {
    TodoRepo{
      pool,
    }
  }

  pub async fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>>{
    let query = sqlx::query_as::<_, Todo>("select * from todos");
    let rows = query.fetch_all(&self.pool).await?;
    Ok(rows)
  }

  pub async fn add(&self, todo: &Todo) -> Result<Todo, Box<dyn Error>>{
    let query = sqlx::query_as::<_, Todo>("INSERT INTO todos (title, description, created_at, last_modified) VALUES ($1, $2, now(), now()) RETURNING *");
    let row = query
                .bind(&todo.title)
                .bind(&todo.description)
                .fetch_one(&self.pool)
                .await?;

    Ok(row)
  }
}
