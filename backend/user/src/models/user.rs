use aws_sdk_dynamodb::{types::{AttributeValue, ReturnValue}, Error};
use std::collections::HashMap;
use uuid::Uuid;

use crate::components::database::Database;

pub struct Model {
    uuid: Uuid,
    username: String,
    password_hash: String,
}

pub struct UpdateAttributes {
    username: Option<String>,
    password_hash: Option<String>,
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
        let uuid = hashmap
            .get("uuid")
            .unwrap()
            .as_s()
            .unwrap()
            .parse()
            .unwrap();
        let username = hashmap.get("username").unwrap().as_s().unwrap().clone();
        let password_hash = hashmap
            .get("password_hash")
            .unwrap()
            .as_s()
            .unwrap()
            .clone();

        Model {
            uuid,
            username,
            password_hash,
        }
    }

    pub async fn create_one(&self, database: &Database) -> Result<Model, Error> {
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
        let user = Self::from_hashmap(attributes);
        Ok(user)
    }

    pub async fn get_one(&self, uuid: Uuid, database: &Database) -> Result<Model, Error> {
        let uuid: AttributeValue = AttributeValue::S(uuid.to_string());

        let request = database
            .client
            .get_item()
            .table_name("users")
            .key("uuid", uuid);

        let response = request.send().await?;

        let attributes = response.item().unwrap();
        let user = Self::from_hashmap(attributes);
        Ok(user)
    }

    pub async fn delete_one(&self, database: &Database) -> Result<(), Error> {
        let uuid: AttributeValue = AttributeValue::S(self.uuid.to_string());

        let request = database
            .client
            .delete_item()
            .table_name("users")
            .key("uuid", uuid);

        request.send().await?;
        Ok(())
    }

    pub async fn update(
        &self,
        new_values: UpdateAttributes,
        database: &Database,
    ) -> Result<Model, Error> {
        let uuid: AttributeValue = AttributeValue::S(self.uuid.to_string());

        let mut attribute_values = HashMap::new();
        let mut update_expression_parts = Vec::new();

        if let Some(username) = new_values.username {
            let username: AttributeValue = AttributeValue::S(username);
            attribute_values.insert(":username", username);
            update_expression_parts.push("username = :username".to_string());
        }

        if let Some(password_hash) = new_values.password_hash {
            let password_hash: AttributeValue = AttributeValue::S(password_hash);
            attribute_values.insert(":password_hash", password_hash);
            update_expression_parts.push("password_hash = :password_hash".to_string());
        }

        let update_expression = update_expression_parts.join(", ");
        let formatted_update_expression = format!("SET {}", update_expression);

        let request = database
            .client
            .update_item()
            .table_name("users")
            .key("uuid", uuid)
            .update_expression(formatted_update_expression)
            .return_values(ReturnValue::AllNew);

        let response = request.send().await?;

        let attributes = response.attributes().unwrap();
        let user = Self::from_hashmap(attributes);
        Ok(user)
    }
}
