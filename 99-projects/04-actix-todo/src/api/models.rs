use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
  title: String,
  description: String,

  #[serde(default = "Utc::now")]
  last_updated: DateTime<Utc>,
}

impl Todo {
  pub fn updated(&mut self) {
    self.last_updated = Utc::now();
  }
}
