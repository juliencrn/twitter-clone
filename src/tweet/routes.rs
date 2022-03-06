use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::response::Response;
use crate::tweet::model::{Tweet, TweetRequest};
use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

#[get("/tweets")]
pub async fn find_all() -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::find_all(50)?;

    Ok(HttpResponse::Ok().json(Response::from(tweets)))
}

#[get("/tweets/{id}")]
pub async fn find(uid: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let tweet_id = uid.into_inner();
    let tweet = Tweet::find(tweet_id)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[post("/tweets")]
pub async fn create(
    tweet_req: web::Json<TweetRequest>,
    _: AuthUser,
) -> Result<HttpResponse, ApiError> {
    let dto = match tweet_req.to_dto() {
        Some(dto) => dto,
        None => return Err(ApiError::new(500, "Unable to create new tweet".to_string())),
    };

    let tweet = Tweet::insert(dto)?;

    Ok(HttpResponse::Created().json(tweet))
}

#[delete("/tweets/{id}")]
pub async fn delete(uid: web::Path<Uuid>, _: AuthUser) -> Result<HttpResponse, ApiError> {
    let tweet_id = uid.into_inner();
    let tweet = Tweet::delete(tweet_id)?;

    Ok(HttpResponse::Ok().json(tweet))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(delete);
}
