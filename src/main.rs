#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod api_error;
pub mod db;
pub mod response;
pub mod schema;
pub mod tweet;
pub mod user;

#[cfg(test)]
mod test;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,twitter=info");
    env_logger::init();
    db::init();

    info!("starting HTTP server...");

    let server = HttpServer::new(move || {
        debug!("Constructing the App");

        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .configure(tweet::init_routes)
            .configure(user::init_routes)
            .default_service(web::to(|| HttpResponse::MethodNotAllowed()))
    })
    .workers(2)
    .bind("127.0.0.1:8080")?;

    info!("Server running on http://localhost:8080");

    server.run().await
}
