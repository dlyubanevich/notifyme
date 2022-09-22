
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId(pub u32);

impl From<u32> for UserId {
    fn from(id: u32) -> Self {
        UserId(id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub user_id: u32,
    pub customer: String,
    pub product: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEventRecord {
    pub timestamp: i64,
    pub user_id: u32,
    pub event: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerEventRecord {
    pub timestamp: i64,
    pub user_id: u32,
    pub customer: Option<String>,
    pub event: String,
    pub data: String,
}
