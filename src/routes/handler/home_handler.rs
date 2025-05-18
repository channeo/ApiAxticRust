use actix_web::{get, post, web, Responder};
use crate::utils::api_response;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
     api_response::ApiResponse::new(200, "test".to_string())
}


#[get("/test")]
async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "test".to_string())
}
