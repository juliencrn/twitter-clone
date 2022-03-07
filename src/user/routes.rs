use crate::auth::{require_owner, Auth};
use crate::errors::ApiError;
use crate::response::Response;
use crate::user::{PublicUser, UpdateUser, User};
use crate::validate::validate;
use actix_web::{delete, get, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users: Vec<PublicUser> = User::find_all()?
        .into_iter()
        .map(|u| PublicUser::from(u))
        .collect::<Vec<PublicUser>>();

    Ok(HttpResponse::Ok().json(Response::from(users)))
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    let user = PublicUser::from(user);

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<Uuid>,
    user: web::Json<UpdateUser>,
    auth: Auth,
) -> Result<HttpResponse, ApiError> {
    validate(&user)?;
    let user_id = id.into_inner();
    require_owner(user_id, auth)?;
    let user = User::update(user_id, user.into_inner())?;
    let user = PublicUser::from(user);

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>, auth: Auth) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();
    require_owner(user_id, auth)?;
    let num_deleted = User::delete(user_id)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

#[get("/profile")]
async fn profile(auth: Auth) -> Result<HttpResponse, ApiError> {
    let user = User::find(auth.id)?;
    let user = PublicUser::from(user);

    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(profile);
}
