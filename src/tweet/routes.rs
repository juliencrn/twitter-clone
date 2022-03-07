use crate::auth::Auth;
use crate::errors::ApiError;
use crate::response::Response;
use crate::tweet::model::{NewTweet, Tweet};
use crate::validate::validate;
use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

#[get("/tweets")]
pub async fn find_all() -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::find_all(50)?;

    Ok(HttpResponse::Ok().json(Response::from(tweets)))
}

#[get("/tweets/{id}")]
pub async fn find(uid: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::find(uid.into_inner())?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[post("/tweets")]
pub async fn create(tweet_req: web::Json<NewTweet>, _: Auth) -> Result<HttpResponse, ApiError> {
    validate(&tweet_req)?;

    let tweet = Tweet::insert(tweet_req.into_inner())?;

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
