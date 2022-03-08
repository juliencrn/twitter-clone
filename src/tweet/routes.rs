use crate::auth::Auth;
use crate::errors::ApiError;
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

    let new_tweet = NewTweet {
        message: tweet_req.into_inner().message,
        author: auth.id,
    };

    let tweet = Tweet::create(new_tweet)?;

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
