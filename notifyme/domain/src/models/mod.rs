use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId(pub u32);

impl From<u32> for UserId {
    fn from(id: u32) -> Self {
        UserId(id)
    }
}
impl From<i64> for UserId {
    fn from(id: i64) -> Self {
        UserId(id as u32)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub name: String,
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
