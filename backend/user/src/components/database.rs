use aws_sdk_dynamodb::Client;
use aws_config::BehaviorVersion;
use std::env;
use dotenv::dotenv;
use crate::models::user;

pub struct Database {
    pub client: aws_sdk_dynamodb::Client,
}

impl Database {
    pub async fn new() -> Self {
        dotenv().ok();

        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "8000".to_string());
        let hostname = format!("{}:{}", host, port);

        let config = aws_config::defaults(BehaviorVersion::latest())
            .test_credentials()
            .load()
            .await;
        let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url(hostname)
            .build();
        let client = Client::from_conf(dynamodb_local_config);
        Database { client }
    }
}