use crate::response::Response;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}
