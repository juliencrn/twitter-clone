use crate::auth::Auth;
use crate::errors::ApiError;
use crate::response::Response;
use crate::user_account::UserAccount;
use actix_web::{get, web, HttpResponse};

/// Return a vec containing the logged-in user accounts
#[get("/accounts")]
async fn find_my_accounts(auth: Auth) -> Result<HttpResponse, ApiError> {
    let accounts = UserAccount::find_by_user(auth.id)?;

    Ok(HttpResponse::Ok().json(Response::from(accounts)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_my_accounts);
    // cfg.service(find);
    // cfg.service(update);
    // cfg.service(delete);
}
