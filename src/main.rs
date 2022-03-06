#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod api_error;
pub mod auth;
pub mod db;
pub mod response;
pub mod routes;
pub mod schema;
pub mod server;
pub mod tweet;
pub mod user;

#[cfg(test)]
mod test;

use crate::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
