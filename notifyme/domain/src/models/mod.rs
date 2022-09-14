use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    name: String,
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    name: String,
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UserEventRecord {
    timestamp: u64, 
    user_id: u32, 
    data: String, 
    comment: String, 
}

#[derive(Serialize, Deserialize)]
pub struct CustomerEventRecord {
    timestamp: u64, 
    user_id: u32, 
    customer_id: u32,
    data: String, 
    comment: String, 
}
