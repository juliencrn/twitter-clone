use crate::db;
use crate::errors::ApiError;
use crate::schema::tweets;
use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Implement relation fields
#[derive(Queryable, Insertable, Deserialize, Serialize, Debug)]
#[table_name = "tweets"]
pub struct Tweet {
    pub id: Uuid,
    pub message: String, // Hello world!
    pub author: Uuid,    // User
    // hashtags: Vec<Uuid>,    // Hashtag[]
    pub likes: i32,             // 86
    pub retweets: i32,          // 6
    pub comments: i32,          // 17
    pub created: NaiveDateTime, // 2020-05-14T12:35:59.209273Z

    // original: Option<Uuid>, // Tweet
    pub asset: String, /* json like:
                       {
                           "id": "fdjnfdsj",
                           "url": "https://res...",
                           "type": "image",
                           "cloudName": "ffdgsgbr"
                       }
                       */
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTweet {
    pub message: String,
    pub author: Uuid,
}

const DEFAULT_LIMIT: i64 = 50;

impl Tweet {
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let tweet = tweets::table
            .filter(tweets::id.eq(id))
            .first::<Tweet>(&conn)?;

        Ok(tweet)
    }

    pub fn find_by_author(user_id: Uuid, limit: Option<i64>) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let results = tweets::table
            .filter(tweets::author.eq(user_id))
            .order(tweets::created.desc())
            .limit(limit.unwrap_or(DEFAULT_LIMIT))
            .load::<Tweet>(&conn)?;

        Ok(results)
    }

    pub fn find_all(limit: Option<i64>) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let results = tweets::table
            .order(tweets::created.desc())
            .limit(limit.unwrap_or(DEFAULT_LIMIT))
            .load::<Tweet>(&conn)?;

        Ok(results)
    }

    pub fn update(id: Uuid, dto: NewTweet) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let tweet = diesel::update(tweets::table.find(id))
            .set(tweets::message.eq(dto.message))
            .get_result::<Tweet>(&conn)?;

        Ok(tweet)
    }

    pub fn create(dto: NewTweet) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let tweet = diesel::insert_into(tweets::table)
            .values(Tweet::from(dto))
            .get_result::<Tweet>(&conn)?;

        Ok(tweet)
    }

    pub fn delete(id: Uuid) -> Result<Tweet, ApiError> {
        let conn = db::connection()?;

        let tweet = diesel::delete(tweets::table.find(id)).get_result(&conn)?;

        Ok(tweet)
    }
}

impl From<NewTweet> for Tweet {
    fn from(dto: NewTweet) -> Self {
        Tweet {
            id: Uuid::new_v4(),
            message: dto.message,
            author: dto.author,
            likes: 0,
            retweets: 0,
            comments: 0,
            created: Utc::now().naive_utc(),
            asset: "".to_string(),
        }
    }
}
