use serde::{Deserialize, Serialize};

use crate::models::{Customer, Notification, Product};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Customers {
        user_id: u32,
        customers: Vec<String>,
    },
    Products {
        user_id: u32,
        products: Vec<String>,
    },
    SubscriptionSuccess {
        user_id: u32,
    },
    SubscriptionFailure {
        user_id: u32,
    },
    CustomerAuthorizationSuccess {
        user_id: u32,
        customer: Customer,
    },
    CustomerAuthorizationFailure {
        user_id: u32,
    },
    Subscriptions {
        user_id: u32,
        products: Vec<String>,
    },
    NewNotification {
        user_id: u32,
        notifications: Vec<Notification>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepositoryResponse {
    Customers {
        user_id: u32,
        customers: Vec<Customer>,
    },
    Products {
        user_id: u32,
        products: Vec<Product>,
    },
    NewSubscription {
        user_id: u32,
        success: bool,
    },
    CustomerAuthorization {
        user_id: u32,
        customer: Option<Customer>,
    },
    Subscriptions {
        user_id: u32,
        products: Vec<Product>,
    },
    NewNotification {
        user_id: u32,
        notifications: Vec<Notification>,
    },
}

impl ToString for Response {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for RepositoryResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
