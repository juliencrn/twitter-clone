use crate::models::like::Like;
use crate::response::Response;
use crate::schema::tweets;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Tweets = Response<Tweet>;

#[derive(Serialize, Queryable)]
pub struct Tweet {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "tweets"]
pub struct TweetDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetDB {
    fn new(message: &str) -> TweetDB {
        TweetDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message: String::from(message),
        }
    }

    pub fn to_tweet(&self) -> Tweet {
        Tweet {
            id: self.id,
            created_at: Utc.from_utc_datetime(&self.created_at),
            message: self.message.to_string(),
            likes: vec![],
        }
    }
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<TweetDB> {
        match &self.message {
            Some(message) => Some(TweetDB::new(message)),
            None => None,
        }
    }
}
