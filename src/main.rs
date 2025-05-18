use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
mod utils;
mod routes;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    
    let port= (*utils::constant::PORT).clone();
    let address  = (*utils::constant::ADDRESS).clone();
    
    
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}