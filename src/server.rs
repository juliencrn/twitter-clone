use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};

use crate::db;
use crate::routes;

pub async fn server() -> io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,twitter=info");
    env_logger::init();
    db::init();

    info!("starting HTTP server...");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default()) // register this first
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default()) // register this last
            .configure(routes::init)
            .default_service(web::to(|| HttpResponse::MethodNotAllowed()))
    })
    .workers(2)
    .bind("127.0.0.1:8080")?;

    info!("Server running on http://localhost:8080");

    server.run().await
}
