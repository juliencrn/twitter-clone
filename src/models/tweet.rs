use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::tweets;
use crate::schema::tweets::dsl::tweets as all_tweets;

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
    /// Create new Tweet struct instance from DTO
    pub fn from<'a>(dto: TweetDto) -> Self {
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

    /// Find one from the database
    pub fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Option<Tweet>> {
        use tweets::dsl::id as uuid;
        all_tweets
            .filter(uuid.eq(id))
            .first::<Tweet>(conn)
            .optional()
    }

    /// Find all tweet ordered by creation date
    pub fn find_all(limit: i64, conn: &PgConnection) -> QueryResult<Vec<Tweet>> {
        all_tweets
            .order(tweets::created.desc())
            .limit(limit)
            .load::<Tweet>(conn)
    }

    /// Update one by uuid and return it
    pub fn update(id: Uuid, conn: &PgConnection, dto: TweetDto) -> QueryResult<Tweet> {
        use tweets::dsl::{asset, message};

        diesel::update(all_tweets.find(id))
            .set((message.eq(dto.message), asset.eq(dto.asset)))
            .get_result::<Tweet>(conn)
    }

    /// Insert a tweet in the database and return it
    pub fn insert(dto: TweetDto, conn: &PgConnection) -> QueryResult<Tweet> {
        diesel::insert_into(tweets::table)
            .values(Tweet::from(dto))
            .get_result::<Tweet>(conn)
    }

    /// Delete one from the database and return it
    pub fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<Tweet> {
        diesel::delete(all_tweets.find(id)).get_result::<Tweet>(conn)
    }

    // pub fn all_by_author(author: String, conn: &PgConnection) -> QueryResult<Vec<Book>> {
    //     all_books
    //         .filter(books::author.eq(author))
    //         .load::<Book>(conn)
    // }
}
