use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use env_logger;

mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Running web server on http://{}\n", address);

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind(address)?
    .run()
    .await
}
