use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;

use crate::handlers::health;

pub mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let hostname = format!("{}:{}", host, port);

    println!("Starting server at {}", hostname);
    HttpServer::new(|| {
        App::new()
            .service(health::handler)
    })
    .bind(hostname)?
    .run()
    .await
}
