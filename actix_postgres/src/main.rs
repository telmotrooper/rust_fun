use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod employees;

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

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
