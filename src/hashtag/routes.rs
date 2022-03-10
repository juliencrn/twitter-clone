use crate::errors::ApiError;
use crate::hashtag::{Hashtag, TweetHashtag};
use crate::response::Response;
use actix_web::{get, web, HttpResponse};

#[get("/hashtags")]
pub async fn list() -> Result<HttpResponse, ApiError> {
    let results = Hashtag::list()?;

    Ok(HttpResponse::Ok().json(Response::from(results)))
}

// TODO: Delete - Only for debug purpose route
#[get("/dev/tweet_hashtags")]
pub async fn dev_list_tweet_hashtags() -> Result<HttpResponse, ApiError> {
    let results = TweetHashtag::list()?;

    Ok(HttpResponse::Ok().json(Response::from(results)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(dev_list_tweet_hashtags);
}
