use crate::api_error::ApiError;
use crate::user::{User, UserDto};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{handle}")]
async fn find(handle: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let user = User::find(&handle)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<UserDto>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{handle}")]
async fn update(
    handle: web::Path<String>,
    user: web::Json<UserDto>,
) -> Result<HttpResponse, ApiError> {
    let user = User::update(&handle, user.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{handle}")]
async fn delete(handle: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let num_deleted = User::delete(&handle)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
