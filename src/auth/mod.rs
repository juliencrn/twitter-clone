mod jwt;
mod model;
mod routes;
mod tests;

pub use jwt::*;
pub use model::*;
pub use routes::init_routes;

// Helpers
use crate::errors::ApiError;

pub fn require_owner(id: uuid::Uuid, auth: AuthUser) -> Result<(), ApiError> {
    if id == auth.id {
        return Ok(());
    }

    Err(ApiError::new(
        401,
        format!("Only the owner can delete this resource"),
    ))
}
