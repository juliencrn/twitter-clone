use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> ApiError {
        let errors = errors
            .field_errors()
            .into_iter()
            .map(|error| {
                let default_error = format!("{} is required", error.0);
                error.1[0]
                    .message
                    .as_ref()
                    .unwrap_or(&std::borrow::Cow::Owned(default_error))
                    .to_string()
            })
            // .collect::<Vec<String>>()
            .next()
            .unwrap_or("Validation error".to_string());

        ApiError::new(422, errors)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                log::error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
