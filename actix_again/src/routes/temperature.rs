use actix_web::{get, web, Responder};
use serde_json::json;

#[get("/temperature")]
async fn temperature() -> impl Responder {
    web::Json(json!({ "temperature": 18 }))
}
