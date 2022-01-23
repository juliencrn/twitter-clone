use crate::models::like::Like;
use crate::response::Response;
use crate::schema::tweets;
use crate::DBPooledConnection;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::{prelude::*, result::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Tweets = Response<Tweet>;

#[derive(Serialize, Queryable)]
pub struct Tweet {
    pub id: String,
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
            id: self.id.to_string(),
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

pub fn find_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let res = tweets.filter(id.eq(_id)).load::<TweetDB>(conn);
    match res {
        Ok(tweets_db) => match tweets_db.first() {
            Some(tweet_db) => Ok(tweet_db.to_tweet()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

pub fn delete_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::tweets::dsl::*;

    let res = diesel::delete(tweets.filter(id.eq(_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn list_tweets(total_tweets: i64, conn: &DBPooledConnection) -> Result<Tweets, Error> {
    use crate::schema::tweets::dsl::*;

    let _tweets = match tweets
        .order(created_at.desc())
        .limit(total_tweets)
        .load::<TweetDB>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Tweets {
        results: _tweets
            .into_iter()
            .map(|t| t.to_tweet())
            .collect::<Vec<Tweet>>(),
    })
}

pub fn create_tweet(tweet: TweetDB, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;
    let _ = diesel::insert_into(tweets).values(&tweet).execute(conn)?;
    Ok(tweet.to_tweet())
}
