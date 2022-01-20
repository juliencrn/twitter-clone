extern crate dotenv;

use actix_web::{get, middleware, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    message: String,
}

/// say "hello world" `/hello`
#[get("/hello")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Message {
            message: "Hello, world!".to_string(),
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
