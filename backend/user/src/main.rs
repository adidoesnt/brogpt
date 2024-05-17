use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;
use crate::handlers::health;
use crate::components::database;

pub mod handlers;
pub mod components;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Initialising database");
    let _db = database::Database::new().await;

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8081".to_string());
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
