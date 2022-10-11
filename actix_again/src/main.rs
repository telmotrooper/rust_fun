use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::json;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {name}!")
}

#[get("/temperature")]
async fn current_temperature() -> impl Responder {
    web::Json(json!({ "temperature": 18 }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello, World!" }))
            .service(greet)
            .service(current_temperature)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
