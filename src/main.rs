use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;

mod controllers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("actix_web start");

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await
}
