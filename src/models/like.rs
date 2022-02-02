use crate::response::Response;
use crate::schema::likes;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub tweet_id: Uuid,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "likes"]
pub struct LikeDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl LikeDB {
    pub fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }

    pub fn to_like(&self) -> Like {
        Like {
            id: self.id,
            created_at: Utc.from_utc_datetime(&self.created_at),
            tweet_id: self.tweet_id,
        }
    }
}
