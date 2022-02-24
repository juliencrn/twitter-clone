#[macro_use]
extern crate diesel;

pub mod api;
pub mod constants;
pub mod db;
pub mod models;
pub mod response;
pub mod schema;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        log::debug!("Constructing the App");

        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(api::tweets::find_all)
            .service(api::tweets::find_one)
            .service(api::tweets::create)
            .service(api::tweets::delete)
            .default_service(web::to(|| HttpResponse::MethodNotAllowed().finish()))
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
