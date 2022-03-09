use crate::auth::Auth;
use crate::errors::ApiError;
use chrono::prelude::*;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_SECRET_KEY: &[u8] = b"secret";
const DAY: usize = 86400000;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: uuid::Uuid,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct WebToken {
    pub token: String,
}

impl From<Claims> for Auth {
    fn from(claims: Claims) -> Self {
        Auth { id: claims.id }
    }
}

pub fn generate_jwt(payload: Auth) -> Result<WebToken, ApiError> {
    let header = Header {
        kid: Some("signing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let Auth { id } = payload;
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        id,
        exp: now + 24 * DAY,
    };

    let token = match encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET_KEY)) {
        Ok(t) => t,
        Err(_) => {
            return Err(ApiError::new(
                500,
                String::from("Unable to generate the JWT"),
            ))
        }
    };

    Ok(WebToken { token })
}

pub fn validate_jwt(token: &str) -> Result<Auth, ApiError> {
    let key = DecodingKey::from_secret(JWT_SECRET_KEY);
    let validation = Validation::new(Algorithm::HS512);

    let token_data = match decode::<Claims>(&token, &key, &validation) {
        Ok(c) => c,
        Err(err) => match &*err.kind() {
            ErrorKind::InvalidToken => {
                return Err(ApiError::new(401, format!("Invalid authentication token")))
            }
            ErrorKind::ExpiredSignature => {
                return Err(ApiError::new(401, format!("JWT signature is expired")))
            }
            e => return Err(ApiError::new(500, format!("JWT decode failed: {:?}", e))),
        },
    };

    Ok(Auth::from(token_data.claims))
}
