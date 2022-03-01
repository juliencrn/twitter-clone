use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api_error::ApiError;
use crate::schema::tweets;

// TODO: Implement relation fields
#[derive(Queryable, Debug, Insertable, Deserialize, Serialize)]
#[table_name = "tweets"]
pub struct Tweet {
    id: Uuid,
    message: String, // Hello world!

    // author: Uuid,           // User
    // hashtags: Vec<Uuid>,    // Hashtag[]
    likes: i32,             // 86
    retweets: i32,          // 6
    comments: i32,          // 17
    created: NaiveDateTime, // 2020-05-14T12:35:59.209273Z

    // original: Option<Uuid>, // Tweet
    asset: String, /* json like:
                   {
                       "id": "fdjnfdsj",
                       "url": "https://res...",
                       "type": "image",
                       "cloudName": "ffdgsgbr"
                   }
                   */
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetDto {
    pub message: String,
    pub asset: String,
}

#[derive(Debug, Deserialize)]
pub struct TweetRequest {
    pub message: Option<String>,
    pub asset: Option<String>,
}

impl TweetRequest {
    pub fn to_dto(&self) -> Option<TweetDto> {
        match &self.message {
            Some(message) => Some(TweetDto {
                message: String::from(message),
                asset: String::new(),
            }),
            None => None,
        }
    }
}

impl Tweet {
    pub fn find(id: Uuid, conn: &PgConnection) -> Result<Self, ApiError> {
        let tweet = tweets::table
            .filter(tweets::id.eq(id))
            .first::<Tweet>(conn)?;

        Ok(tweet)
    }

    pub fn find_all(limit: i64, conn: &PgConnection) -> Result<Vec<Self>, ApiError> {
        let all_tweets = tweets::table
            .order(tweets::created.desc())
            .limit(limit)
            .load::<Tweet>(conn)?;

        Ok(all_tweets)
    }

    pub fn update(id: Uuid, conn: &PgConnection, dto: TweetDto) -> Result<Self, ApiError> {
        let tweet = diesel::update(tweets::table.find(id))
            .set((tweets::message.eq(dto.message), tweets::asset.eq(dto.asset)))
            .get_result::<Tweet>(conn)?;

        Ok(tweet)
    }

    pub fn insert(dto: TweetDto, conn: &PgConnection) -> Result<Self, ApiError> {
        let tweet = diesel::insert_into(tweets::table)
            .values(Tweet::from(dto))
            .get_result::<Tweet>(conn)?;

        Ok(tweet)
    }

    pub fn delete(id: Uuid, conn: &PgConnection) -> Result<Self, ApiError> {
        let tweet = diesel::delete(tweets::table.find(id)).get_result::<Tweet>(conn)?;

        Ok(tweet)
    }
}

impl From<TweetDto> for Tweet {
    fn from(dto: TweetDto) -> Self {
        Tweet {
            id: Uuid::new_v4(),
            message: dto.message,
            likes: 0,
            retweets: 0,
            comments: 0,
            created: Utc::now().naive_utc(),
            asset: dto.asset,
        }
    }
}
