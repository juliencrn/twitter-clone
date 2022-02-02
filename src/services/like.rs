use crate::models::like::{Like, LikeDB, Likes};
use crate::services;
use crate::DBPooledConnection;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

pub fn list_likes(
    tweet_id: &Uuid,
    total_likes: i64,
    conn: &DBPooledConnection,
) -> Result<Likes, Error> {
    use crate::schema::likes::dsl;

    let likes = match dsl::likes
        .filter(dsl::tweet_id.eq(tweet_id))
        .order(dsl::created_at.desc())
        .limit(total_likes)
        .load::<LikeDB>(conn)
    {
        Ok(lks) => lks,
        Err(_) => vec![],
    };

    Ok(Likes {
        results: likes
            .into_iter()
            .map(|t| t.to_like())
            .collect::<Vec<Like>>(),
    })
}

pub fn create_like(tweet_id: Uuid, conn: &DBPooledConnection) -> Result<Like, Error> {
    use crate::schema::likes::dsl;

    // 1. find tweet
    let tweet = services::tweet::find_tweet(tweet_id, conn)?;

    // 2. Save new like
    let like_db = LikeDB::new(tweet.id);
    diesel::insert_into(dsl::likes)
        .values(&like_db)
        .execute(conn)?;

    Ok(like_db.to_like())
}

pub fn delete_like(id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::likes::dsl;

    match diesel::delete(dsl::likes.filter(dsl::id.eq(id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
