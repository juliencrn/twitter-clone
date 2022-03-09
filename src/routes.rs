use crate::auth;
use crate::tweet;
use crate::user;
use crate::user_account;

use actix_web::web::{scope, ServiceConfig};

// app routes without prefix in a separate fn for testing purpose
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::init_routes));
    cfg.configure(user::init_routes);
    cfg.configure(user_account::init_routes);
    cfg.configure(tweet::init_routes);
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").configure(routes));
}
