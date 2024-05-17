use uuid::{Uuid};

struct Model {
    uuid: Uuid,
    username: String,
    email: String,
    password_hash: String,
}