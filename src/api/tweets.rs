use crate::constants::CONNECTION_POOL_ERROR;
use crate::db::DBPool;
use crate::models::{Tweet, TweetRequest};
use crate::response::Response;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

use std::str::FromStr;
use uuid::Uuid;

type Pool = web::Data<DBPool>;

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn find_all(pool: Pool) -> impl Responder {
    // use web::block to offload blocking Diesel code without blocking server thread
    let tweets = web::block(move || {
        let conn = pool.get().expect(CONNECTION_POOL_ERROR);
        Tweet::find_all(50, &conn)
    })
    .await;

    match tweets {
        Ok(tweets) => HttpResponse::Ok().json(Response::from(tweets)),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: web::Json<TweetRequest>, pool: Pool) -> impl Responder {
    let dto = tweet_req.to_dto().expect("Unable to create new tweet");

    // use web::block to offload blocking Diesel code without blocking server thread
    let created = web::block(move || {
        let conn = pool.get().expect(CONNECTION_POOL_ERROR);
        Tweet::insert(dto, &conn)
    })
    .await;

    match created {
        Ok(tweet) => HttpResponse::Created().json(tweet),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn find_one(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(path.0 .0.as_str()) {
        // use web::block to offload blocking Diesel code without blocking server thread
        let tweet = web::block(move || {
            let conn = pool.get().expect(CONNECTION_POOL_ERROR);
            Tweet::find(uuid, &conn)
        })
        .await;

        // match tweet {}

        if let Ok(tweet) = tweet {
            HttpResponse::Ok().json(tweet)
        } else {
            HttpResponse::NoContent().await.unwrap()
        }
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: web::Path<(String,)>, pool: web::Data<DBPool>) -> impl Responder {
    if let Ok(uuid) = Uuid::from_str(path.0 .0.as_str()) {
        // use web::block to offload blocking Diesel code without blocking server thread
        let result = web::block(move || {
            let conn = pool.get().expect(CONNECTION_POOL_ERROR);
            Tweet::delete(uuid, &conn)
        })
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().await.unwrap(),
            Err(_) => HttpResponse::NoContent().await.unwrap(),
        }
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}