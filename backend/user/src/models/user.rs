use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use uuid::Uuid;

struct Model {
    uuid: Uuid,
    username: String,
    email: String,
    password_hash: String,
}

impl Model {
    pub fn to_attr_map(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_string()));
        item.insert(
            "username".to_string(),
            AttributeValue::S(self.username.clone()),
        );
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item.insert(
            "password_hash".to_string(),
            AttributeValue::S(self.password_hash.clone()),
        );
        item
    }

    pub fn from_attr_map(item: HashMap<String, AttributeValue>) -> Option<Self> {
        let uuid = item.get("uuid")?.as_s().ok()?.to_string();
        let username = item.get("username")?.as_s().ok()?.to_string();
        let email = item.get("email")?.as_s().ok()?.to_string();
        let password_hash = item.get("password_hash")?.as_s().ok()?.to_string();
        Some(Self {
            uuid: Uuid::parse_str(&uuid).ok()?,
            username,
            email,
            password_hash,
        })
    }
}
