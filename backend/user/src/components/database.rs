use aws_sdk_dynamodb::Client;
use aws_config::BehaviorVersion;
use std::env;
use dotenv::dotenv;

pub struct Database {
    pub client: aws_sdk_dynamodb::Client,
}

impl Database {
    pub async fn new() -> Self {
        dotenv().ok();

        let host: String = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port: String = env::var("DB_PORT").unwrap_or_else(|_| "8000".to_string());
        let hostname: String = format!("{}:{}", host, port);

        let config: aws_config::SdkConfig = aws_config::defaults(BehaviorVersion::latest())
            .test_credentials()
            .load()
            .await;
        let dynamodb_local_config: aws_sdk_dynamodb::Config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url(hostname)
            .build();
        let client: Client = Client::from_conf(dynamodb_local_config);
        Database { client }
    }
}