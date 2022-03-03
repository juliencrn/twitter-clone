use crate::api_error::ApiError;
use crate::response::Response;
use crate::user::{User, UserDto};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;

    Ok(HttpResponse::Ok().json(Response::from(users)))
}

#[get("/users/{handle}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();
    let user = User::find(user_id)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<UserDto>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{handle}")]
async fn update(id: web::Path<Uuid>, user: web::Json<UserDto>) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();
    let user = User::update(user_id, user.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{handle}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();
    let num_deleted = User::delete(user_id)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
