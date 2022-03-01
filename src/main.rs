#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod api_error;
pub mod constants;
pub mod db;
pub mod response;
pub mod schema;
pub mod tweet;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = get_db_url().expect("POSTGRES_* env variable(s) missing");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    info!("starting HTTP server...");

    let server = HttpServer::new(move || {
        debug!("Constructing the App");

        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .configure(tweet::init_routes)
            .default_service(web::to(|| HttpResponse::MethodNotAllowed().finish()))
    })
    .workers(2)
    .bind("127.0.0.1:8080")?;

    info!("Server running on http://localhost:8080");

    server.run().await
}

fn get_db_url() -> Result<String, std::env::VarError> {
    use std::env::var;

    Ok(format!(
        "postgres://{}:{}@localhost:5432/{}",
        var("POSTGRES_USER")?,
        var("POSTGRES_PASSWORD")?,
        var("POSTGRES_DB")?
    ))
}
