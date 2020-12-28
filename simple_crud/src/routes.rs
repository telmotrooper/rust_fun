use actix_web::{HttpResponse, get, Responder};
use actix_web::web::ServiceConfig;

use std::env;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/db-test")]
async fn db_test() -> impl Responder {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set.");

    HttpResponse::Ok().body(database_url)
}
pub fn init_routes(config: &mut ServiceConfig) {
    config.service(home);
    config.service(db_test);
}
