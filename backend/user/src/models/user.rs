use aws_sdk_dynamodb::{Error, types::AttributeValue};
use uuid::Uuid;
use std::collections::HashMap;

use crate::components::database::Database;

pub struct Model {
    uuid: Uuid,
    username: String,
    password_hash: String,
}

impl Model {
    pub fn new(username: String, password_hash: String) -> Self {
        Model {
            uuid: Uuid::new_v4(),
            username,
            password_hash,
        }
    }

    pub fn from_hashmap(hashmap: &HashMap<String, AttributeValue>) -> Self {
        let uuid = hashmap.get("uuid").unwrap().as_s().unwrap().parse().unwrap();
        let username = hashmap.get("username").unwrap().as_s().unwrap().clone();
        let password_hash = hashmap.get("password_hash").unwrap().as_s().unwrap().clone();

        Model {
            uuid,
            username,
            password_hash,
        }
    }

    pub async fn create_one(&self, database: &Database) -> Result<Model, Error>{
        let uuid: AttributeValue = AttributeValue::S(self.uuid.to_string());
        let username: AttributeValue = AttributeValue::S(self.username.clone());
        let password_hash: AttributeValue = AttributeValue::S(self.password_hash.clone());

        let request = database
            .client
            .put_item()
            .table_name("users")
            .item("uuid", uuid)
            .item("username", username)
            .item("password_hash", password_hash);

        let response = request.send().await?;
        
        let attributes = response.attributes().unwrap();
        let user = Model::from_hashmap(attributes);
        Ok(user)
    }
}
