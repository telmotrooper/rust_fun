#![allow(unused_imports)]

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod employees;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(employees::init_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
