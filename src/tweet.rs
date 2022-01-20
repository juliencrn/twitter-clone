use actix_web::{delete, get, post, web, HttpResponse};
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::like::Like;

#[derive(Deserialize, Serialize)]
pub struct Tweets {
    results: Vec<Tweet>,
}

const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    #[serde(serialize_with = "to_ts")]
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list() -> HttpResponse {
    // TODO find the last 50 tweets and return them

    let tweets = Tweets { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: web::Json<TweetRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(tweet_req.to_tweet())
}

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn get(path: web::Path<(String,)>) -> HttpResponse {
    // TODO find tweet a tweet by ID and return it
    let found_tweet: Option<Tweet> = None;

    match found_tweet {
        Some(tweet) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: web::Path<(String,)>) -> HttpResponse {
    // TODO delete tweet by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
