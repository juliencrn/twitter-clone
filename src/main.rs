extern crate dotenv;

mod like;
mod tweet;

use actix_web::{middleware, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::list)
            .service(tweet::get)
            .service(tweet::create)
            .service(tweet::delete)
            .service(like::list)
            .service(like::plus_one)
            .service(like::minus_one)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
