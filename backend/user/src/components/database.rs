use aws_sdk_dynamodb::Client;
use aws_config::BehaviorVersion;

pub struct Database {
    pub client: aws_sdk_dynamodb::Client,
}

impl Database {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .test_credentials()
            .load()
            .await;
        let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url("http://localhost:8000")
            .build();
        let client = Client::from_conf(dynamodb_local_config);
        Database { client }
    }
}