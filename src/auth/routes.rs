use crate::auth::{generate_jwt, AuthUser, Credentials};
use crate::errors::ApiError;
use crate::user::{NewUser, User};
use crate::validate::validate;
use actix_web::{post, web, HttpResponse};

#[post("/register")]
async fn register(user: web::Json<NewUser>) -> Result<HttpResponse, ApiError> {
    validate(&user)?;

    let user = User::create(user.into_inner())?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(credentials: web::Json<Credentials>) -> Result<HttpResponse, ApiError> {
    let Credentials { handle, password } = credentials.into_inner();
    let user: User = User::find_by_handle(&handle)?;
    let is_valid = user.verify_password(&password)?;
    let token = generate_jwt(AuthUser::from(user))?;

    match is_valid {
        true => Ok(HttpResponse::Ok().json(token)),
        false => Err(ApiError::new(401, String::from("Credentials not valid!"))),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
