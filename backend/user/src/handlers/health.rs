use actix_web::{web, get, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: u16,
    message: String,
}

#[get("/health")]
async fn handler() -> impl Responder {
    let message: &str = "Server is healthy!";
    let response_body = web::Json(HealthResponse {
        status: 200,
        message: message.to_string(),
    });
    HttpResponse::Ok().json(response_body)
}