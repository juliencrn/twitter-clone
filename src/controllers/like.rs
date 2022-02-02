use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::{like::*, tweet};
use crate::services;
use crate::DBPool;
use actix_web::{delete, get, post, web, HttpResponse};
use std::str::FromStr;
use uuid::Uuid;

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweet_id = Uuid::from_str(path.0 .0.as_str()).unwrap();
    let likes = web::block(move || services::like::list_likes(&tweet_id, 10, &conn)).await;

    match likes {
        Ok(likes) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(likes),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

/// add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweet_id = Uuid::from_str(path.0 .0.as_str()).unwrap();
    match services::like::create_like(tweet_id, &conn) {
        Ok(like) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(like),
        Err(_) => HttpResponse::BadRequest().await.unwrap(),
    }
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweet_id = Uuid::from_str(path.0 .0.as_str()).unwrap();
    match services::like::delete_like(tweet_id, &conn) {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}
