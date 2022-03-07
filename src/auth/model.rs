use crate::auth::validate_jwt;
use crate::errors::ApiError;
use crate::user::User;
use actix_web::{dev::Payload, http, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth {
    pub id: Uuid,
    pub handle: String,
}

impl From<User> for Auth {
    fn from(user: User) -> Self {
        Auth {
            id: user.id,
            handle: user.handle,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Credentials {
    pub handle: String,
    pub password: String,
}

/// Extractor for pulling the identity out of a request.
///
/// More than just an extractor, it returns an 401 when invocated if the token if auth invalid
/// Simply add "user: Auth" to a handler to invoke this.
impl FromRequest for Auth {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers();

        if let Some(authorization) = headers.get(http::header::AUTHORIZATION) {
            if let Ok(authorization_str) = authorization.to_str() {
                if let Some((_, token)) = authorization_str.split_once(" ") {
                    if let Ok(auth_user) = validate_jwt(token) {
                        return ok(auth_user);
                    }
                }
            }
        }

        err(ApiError::new(401, format!("Unauthorized")))
    }
}
