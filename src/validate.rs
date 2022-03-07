//! Validation-related functions to work with the validator crate.

use crate::errors::ApiError;
use actix_web::web::Json;
use validator::Validate;

/// Validate a struct and collect and return the errors
pub fn validate<T>(params: &Json<T>) -> Result<(), ApiError>
where
    T: Validate,
{
    if let Err(errors) = params.validate() {
        return Err(ApiError::from(errors));
    }

    Ok(())
}
