use crate::models::tweet::*;
use crate::DBPooledConnection;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

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
