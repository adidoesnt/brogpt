use actix_web::{get, App, HttpServer, Responder, HttpResponse, web};
use std::env;
use dotenv::dotenv;
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: u16,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    let message: &str = "Server is healthy!";
    let response_body = web::Json(Health {
        status: 200,
        message: message.to_string(),
    });
    HttpResponse::Ok().json(response_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let hostname = format!("{}:{}", host, port);

    println!("Starting server at {}", hostname);
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(hostname)?
    .run()
    .await
}
