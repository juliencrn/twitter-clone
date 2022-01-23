use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::like::*;
use crate::DBPool;
use actix_web::{delete, get, post, web, HttpResponse};

// use std::str::FromStr;
// use uuid::Uuid;

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    // TODO find likes by tweet ID and return them
    let likes = Likes { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

/// add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    // TODO add one like to a tweet
    let like = Like::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    // TODO remove one like to a tweet
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
