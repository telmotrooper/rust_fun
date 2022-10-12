use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load variables from ".env" file.

    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    println!("Starting server at: http://localhost:{port}");
    
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello, World!" }))
            .service(routes::hello)
            .service(routes::temperature)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
