use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::repo::{Status, Todo};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoRequest {
  title: String,
  description: String,
}

impl Into<Todo> for CreateTodoRequest {
  fn into(self) -> Todo {
      Todo{
        title: self.title,
        description: self.description,
        created_at: None,
        last_modified: None,
        status: None,
      }
  }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoResponse {
  title: String,
  description: String,
  last_modified: DateTime<Utc>,
  created_at: DateTime<Utc>,
  status: Status
}

impl From<Todo> for CreateTodoResponse {
  fn from(todo: Todo) -> Self {
    CreateTodoResponse {
        title: todo.title,
        description: todo.description,
        created_at: todo.created_at.unwrap(),
        last_modified: todo.last_modified.unwrap(),
        status: todo.status.unwrap(),
      }
  }
}
