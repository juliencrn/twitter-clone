#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod auth;
pub mod db;
pub mod errors;
pub mod response;
pub mod routes;
pub mod schema;
pub mod server;
pub mod tweet;
pub mod user;
pub mod user_account;
pub mod validate;

#[cfg(test)]
mod test;

use crate::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
