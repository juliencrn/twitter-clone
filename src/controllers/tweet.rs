use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::tweet::*;
use crate::DBPool;
use actix_web::{delete, get, post, web, HttpResponse};

use std::str::FromStr;
use uuid::Uuid;

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    match list_tweets(50, &conn) {
        Ok(tweets) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweets),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: web::Json<TweetRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let new_tweet = tweet_req.to_tweet().expect("Unable to create new tweet");
    let created = create_tweet(new_tweet, &conn);

    match created {
        Ok(tweet) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn get(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    match Uuid::from_str(path.0 .0.as_str()) {
        Ok(uuid) => match find_tweet(uuid, &conn) {
            Ok(tweet) => HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(tweet),
            _ => HttpResponse::NoContent()
                .content_type(APPLICATION_JSON)
                .await
                .unwrap(),
        },
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    match Uuid::from_str(path.0 .0.as_str()) {
        Ok(uuid) => match delete_tweet(uuid, &conn) {
            Ok(_) => HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .await
                .unwrap(),
            _ => HttpResponse::NoContent()
                .content_type(APPLICATION_JSON)
                .await
                .unwrap(),
        },
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}
