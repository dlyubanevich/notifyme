use serde::{Deserialize, Serialize};

use crate::models::{Customer, Product};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Customers {
        user_id: u32,
    },
    Products {
        user_id: u32,
        customer: String,
    },
    NewSubscription {
        user_id: u32,
        customer: String,
        product: String,
    },
    CustomerAuthorization {
        user_id: u32,
        key: String,
    },
    Subscriptions {
        user_id: u32,
        customer: Customer,
    },
    NewNotification {
        user_id: u32,
        customer: Customer,
        product: Product,
        notification: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepositoryRequest {
    Customers {
        user_id: u32,
    },
    Products {
        user_id: u32,
        customer_id: u32,
    },
    NewSubscription {
        user_id: u32,
        customer_id: u32,
        product_id: u32,
    },
    CustomerAuthorization {
        user_id: u32,
        key: String,
    },
    Subscriptions {
        user_id: u32,
        customer_id: u32,
    },
    NewNotification {
        user_id: u32,
        customer_id: u32,
        product_id: u32,
        notification: String,
    },
}

impl ToString for Request {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for RepositoryRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
