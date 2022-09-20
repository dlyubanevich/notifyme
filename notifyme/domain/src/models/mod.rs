use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub name: String,
    pub id: u32,
}

impl Customer {
    pub fn new(name: &str) -> Self {
        Customer { name: name.to_owned(), id: 1 }   
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub name: String,
    pub id: u32,
}

impl Product {
    pub fn new(name: &str) -> Self {
        Product { name: name.to_owned(), id: 1 }   
    }
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

impl FromStr for Product {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).unwrap()
    }
}

impl From<Customer> for String {
    fn from(s: Customer) -> Self {
        serde_json::to_string(&s).unwrap()
    }
}