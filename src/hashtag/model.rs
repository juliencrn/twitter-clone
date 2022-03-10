use crate::db;
use crate::errors::ApiError;
use crate::schema::{hashtags, tweet_hashtags};
use crate::tweet::Tweet;
use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Deserialize, Serialize, Debug, Identifiable, Associations)]
#[table_name = "hashtags"]
pub struct Hashtag {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewHashtag<'a> {
    pub name: &'a str,
}

/// Many-to-many relation struct
#[derive(Identifiable, Queryable, Associations, Insertable, Serialize)]
#[belongs_to(Tweet)]
#[belongs_to(Hashtag)]
#[table_name = "tweet_hashtags"]
pub struct TweetHashtag {
    pub id: Uuid,
    pub hashtag_id: Uuid,
    pub tweet_id: Uuid,
}

pub struct NewTweetHashtag {
    pub hashtag_id: Uuid,
    pub tweet_id: Uuid,
}

impl Hashtag {
    pub fn create(new_hashtag: NewHashtag, tweet_id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        // Create the hashtag
        let hashtag = diesel::insert_into(hashtags::table)
            .values(Hashtag::from(new_hashtag))
            .get_result::<Hashtag>(&conn)?;

        // Save the relation
        let new_th = NewTweetHashtag {
            hashtag_id: hashtag.id,
            tweet_id,
        };

        diesel::insert_into(tweet_hashtags::table)
            .values(TweetHashtag::from(new_th))
            .get_result::<TweetHashtag>(&conn)?;

        Ok(hashtag)
    }

    pub fn list() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let results = hashtags::table
            .order(hashtags::created.desc())
            .limit(50)
            .load::<Hashtag>(&conn)?;

        Ok(results)
    }
}

impl TweetHashtag {
    pub fn create(new_th: NewTweetHashtag) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let th = diesel::insert_into(tweet_hashtags::table)
            .values(TweetHashtag::from(new_th))
            .get_result::<TweetHashtag>(&conn)?;

        Ok(th)
    }

    pub fn list() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let results = tweet_hashtags::table
            .limit(50)
            .load::<TweetHashtag>(&conn)?;

        Ok(results)
    }
}

impl<'a> From<NewHashtag<'a>> for Hashtag {
    fn from(new_hashtag: NewHashtag) -> Self {
        Hashtag {
            id: Uuid::new_v4(),
            name: new_hashtag.name.to_string(),
            created: Utc::now().naive_utc(),
        }
    }
}

impl From<NewTweetHashtag> for TweetHashtag {
    fn from(new_th: NewTweetHashtag) -> Self {
        TweetHashtag {
            id: Uuid::new_v4(),
            tweet_id: new_th.tweet_id,
            hashtag_id: new_th.hashtag_id,
        }
    }
}
