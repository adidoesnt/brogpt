use aws_sdk_dynamodb as dynamodb;
use aws_config::{load_defaults, BehaviorVersion};

pub struct Database {
    pub client: aws_sdk_dynamodb::Client,
}

impl Database {
    pub async fn new() -> Self {
        let config = load_defaults(BehaviorVersion::v2024_03_28()).await;
        let client = dynamodb::Client::new(&config);
        Database { client }
    }
}