use crate::auth::{generate_jwt, Auth};
use crate::errors::ApiError;
use crate::user::{NewUser, User};
use crate::user_account::{NewUserAccount, UserAccount};
use crate::validate::validate;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        message = "handle is required and must be at least 3 characters"
    ))]
    pub handle: String,

    #[validate(email(message = "email is required and must be valid email"))]
    pub email: String,

    #[validate(length(
        min = 8,
        message = "password is required and must be at least 8 characters"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/register")]
async fn register(req: web::Json<RegisterRequest>) -> Result<HttpResponse, ApiError> {
    validate(&req)?;

    let RegisterRequest {
        name,
        handle,
        email,
        password,
    } = req.into_inner();

    if let Ok(_) = UserAccount::find_by_email(&email) {
        return Err(ApiError::new(422, format!("email already taken")));
    }

    let user = User::create(NewUser { name, handle })?;

    UserAccount::create(NewUserAccount {
        user_id: user.id,
        email,
        password,
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(credentials: web::Json<LoginRequest>) -> Result<HttpResponse, ApiError> {
    let LoginRequest { email, password } = credentials.into_inner();

    // Don't return errors directly to hide why it doesn't work
    if let Ok(account) = UserAccount::find_by_email(&email) {
        if let Ok(is_valid) = account.verify_password(&password) {
            let token = generate_jwt(Auth {
                id: account.user_id,
            })?;
            if is_valid {
                return Ok(HttpResponse::Ok().json(token));
            }
        }
    }

    Err(ApiError::new(401, String::from("Credentials not valid!")))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
