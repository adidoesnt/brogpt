use aws_sdk_dynamodb::{types::{AttributeValue, ReturnValue}, Error};
use std::collections::HashMap;

use crate::components::database::Database;

pub struct Model {
    username: String,
    password_hash: String,
}

pub struct UpdateAttributes {
    password_hash: Option<String>,
}

impl Model {
    pub fn new(username: String, password_hash: String) -> Self {
        Model {
            username,
            password_hash,
        }
    }

    pub fn from_hashmap(hashmap: &HashMap<String, AttributeValue>) -> Self {
        let username = hashmap.get("username").unwrap().as_s().unwrap().clone();
        let password_hash = hashmap
            .get("password_hash")
            .unwrap()
            .as_s()
            .unwrap()
            .clone();

        Model {
            username,
            password_hash,
        }
    }

    pub async fn create_one(&self, database: &Database) -> Result<Model, Error> {
        let username: AttributeValue = AttributeValue::S(self.username.to_string());
        let password_hash: AttributeValue = AttributeValue::S(self.password_hash.to_string());

        let request = database
            .client
            .put_item()
            .table_name("users")
            .item("username", username)
            .item("password_hash", password_hash);

        let response = request.send().await?;

        let attributes = response.attributes().unwrap();
        let user = Self::from_hashmap(attributes);
        Ok(user)
    }

    pub async fn get_one(&self, username: String, database: &Database) -> Result<Model, Error> {
        let username: AttributeValue = AttributeValue::S(username.to_string());

        let request = database
            .client
            .get_item()
            .table_name("users")
            .key("username", username);

        let response = request.send().await?;

        let attributes = response.item().unwrap();
        let user = Self::from_hashmap(attributes);
        Ok(user)
    }

    pub async fn delete_one(&self, database: &Database) -> Result<(), Error> {
        let username: AttributeValue = AttributeValue::S(self.username.to_string());

        let request = database
            .client
            .delete_item()
            .table_name("users")
            .key("username", username);

        request.send().await?;
        Ok(())
    }

    pub async fn update(
        &self,
        new_values: UpdateAttributes,
        database: &Database,
    ) -> Result<Option<Model>, Error> {
        let username: AttributeValue = AttributeValue::S(self.username.to_string());

        let mut attribute_values = HashMap::new();
        let mut update_expression_parts = Vec::new();

        if let Some(password_hash) = new_values.password_hash {
            let password_hash: AttributeValue = AttributeValue::S(password_hash);
            attribute_values.insert(":password_hash", password_hash);
            update_expression_parts.push("password_hash = :password_hash".to_string());
        }

        if update_expression_parts.len() == 0 {
            return Ok(None);
        }

        let update_expression = update_expression_parts.join(", ");
        let formatted_update_expression = format!("SET {}", update_expression);

        let request = database
            .client
            .update_item()
            .table_name("users")
            .key("username", username)
            .update_expression(formatted_update_expression)
            .return_values(ReturnValue::AllNew);

        let response = request.send().await?;

        let attributes = response.attributes().unwrap();
        let user = Self::from_hashmap(attributes);
        Ok(Some(user))
    }
}
