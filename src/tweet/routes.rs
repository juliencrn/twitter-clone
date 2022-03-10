use crate::auth::Auth;
use crate::errors::ApiError;
use crate::hashtag::{Hashtag, NewHashtag};
use crate::response::Response;
use crate::tweet::model::{NewTweet, Tweet};
use crate::validate::validate;
use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewTweetRequest {
    #[validate(length(min = 1, message = "tweet message is missing"))]
    pub message: String,

    #[validate]
    pub hashtags: Option<Vec<RequestHashtag>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RequestHashtag {
    #[validate(length(min = 3, message = "hashtag must be at least 3 characters"))]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FindParams {
    user_id: Option<Uuid>,
    limit: Option<i64>,
}

#[get("/tweets")]
pub async fn find_all(query: web::Query<FindParams>) -> Result<HttpResponse, ApiError> {
    let tweets = match query.user_id {
        Some(user_id) => Tweet::find_by_author(user_id, query.limit),
        None => Tweet::find_all(query.limit),
    }?;

    Ok(HttpResponse::Ok().json(Response::from(tweets)))
}

#[get("/tweets/{id}")]
pub async fn find(uid: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::find(uid.into_inner())?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[post("/tweets")]
pub async fn create(
    tweet_req: web::Json<NewTweetRequest>,
    auth: Auth,
) -> Result<HttpResponse, ApiError> {
    validate(&tweet_req)?;

    let NewTweetRequest { message, hashtags } = tweet_req.into_inner();

    let tweet = Tweet::create(NewTweet {
        message: &message,
        author: auth.id,
    })?;

    // If there is hashtags, save it
    if let Some(req_hashtags) = hashtags {
        for req_hashtag in req_hashtags {
            let new_hashtag = NewHashtag {
                name: &req_hashtag.name,
            };
            Hashtag::create(new_hashtag, tweet.id)?;
        }
    }

    Ok(HttpResponse::Created().json(tweet))
}

#[delete("/tweets/{id}")]
pub async fn delete(uid: web::Path<Uuid>, _: Auth) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::delete(uid.into_inner())?;

    Ok(HttpResponse::Ok().json(tweet))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(delete);
}
