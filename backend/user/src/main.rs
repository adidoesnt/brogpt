use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use dotenv::dotenv;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
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
            .route("/", web::get().to(index))
    })
    .bind(hostname)?
    .run()
    .await
}
