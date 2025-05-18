use actix_web::web;

use super::handler;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/home"))
    .service(handler::home_handler::greet);
}