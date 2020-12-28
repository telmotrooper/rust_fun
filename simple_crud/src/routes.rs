use actix_web::{HttpResponse, get, Responder};
use actix_web::web::ServiceConfig;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(home);
}
