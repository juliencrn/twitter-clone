use crate::api_error::ApiError;
use crate::constants::CONNECTION_POOL_ERROR;
use crate::db::DBPool;
use crate::response::Response;
use crate::tweet::model::{Tweet, TweetRequest};
use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

type Pool = web::Data<DBPool>;
type ResponseResult = Result<HttpResponse, ApiError>;

#[get("/tweets")]
pub async fn find_all(pool: Pool) -> ResponseResult {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets = Tweet::find_all(50, &conn)?;

    Ok(HttpResponse::Ok().json(Response::from(tweets)))
}

#[get("/tweets/{id}")]
pub async fn find(uid: web::Path<Uuid>, pool: web::Data<DBPool>) -> ResponseResult {
    let tweet_id = uid.into_inner();
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweet = Tweet::find(tweet_id, &conn)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[post("/tweets")]
pub async fn create(tweet_req: web::Json<TweetRequest>, pool: Pool) -> ResponseResult {
    let dto = match tweet_req.to_dto() {
        Some(dto) => dto,
        None => return Err(ApiError::new(500, "Unable to create new tweet".to_string())),
    };

    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweet = Tweet::insert(dto, &conn)?;

    Ok(HttpResponse::Created().json(tweet))
}

#[delete("/tweets/{id}")]
pub async fn delete(uid: web::Path<Uuid>, pool: web::Data<DBPool>) -> ResponseResult {
    let tweet_id = uid.into_inner();

    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweet = Tweet::delete(tweet_id, &conn)?;

    Ok(HttpResponse::Ok().json(tweet))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(delete);
}
