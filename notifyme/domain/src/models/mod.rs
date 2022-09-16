use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub name: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub name: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub user_id: u32,
    pub customer: Customer,
    pub product: Product,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEventRecord {
    pub timestamp: f64,
    pub user_id: u32,
    pub event: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerEventRecord {
    pub timestamp: f64,
    pub user_id: u32,
    pub customer_id: u32,
    pub event: String,
    pub data: String,
}
