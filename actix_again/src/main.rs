use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello, World!" }))
            .service(routes::hello)
            .service(routes::temperature)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
