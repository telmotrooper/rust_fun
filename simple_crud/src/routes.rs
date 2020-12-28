use actix_web::{HttpResponse, get, Responder};
use actix_web::web::ServiceConfig;
use sqlx::postgres::PgPoolOptions;

use std::env;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/db-test")]
async fn db_test() -> impl Responder {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await.ok()?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await.ok()?;

    Some(row.0.to_string())
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(home);
    config.service(db_test);
}
