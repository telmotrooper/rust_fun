use actix_web::{get, web, Responder};
use reqwest;
use serde_json::json;

#[get("/cat")]
pub async fn cat() -> impl Responder {
    let response = reqwest::get("https://catfact.ninja/fact").await;

    // Still not returning JSON correctly for successful response.
    match response {
        Ok(v) => web::Json(json!(v.text().await.unwrap())),
        Err(_) => web::Json(json!({ "error": true })),
    }
}
