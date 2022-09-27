use serde::{Deserialize, Serialize};

use crate::models::{Customer, Notification, Product, UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientResponse {
    Customers {
        user_id: UserId,
        customers: Vec<Customer>,
    },
    Products {
        user_id: UserId,
        products: Vec<Product>,
    },
    SubscriptionSuccess {
        user_id: UserId,
    },
    SubscriptionFailure {
        user_id: UserId,
    },
    CustomerNotification {
        user_id: UserId,
        customer: String,
        product: String,
        notification: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomerResponse {
    AuthorizationSuccess {
        user_id: UserId,
        customer: Customer,
    },
    AuthorizationFailure {
        user_id: UserId,
    },
    ProductsForNotification {
        user_id: UserId,
        customer: String,
        products: Vec<Product>,
    },
    NotificationSuccess {
        user_id: UserId,
    },
    NotificationFailure {
        user_id: UserId,
    },
    ClientSubscription {
        user_id: UserId,
        customer: String,
        product: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientResponseFromRepository {
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomerResponseFromRepository {
    Authorization {
        user_id: u32,
        customer: Option<Customer>,
    },
    ProductsForNotification {
        user_id: u32,
        customer: String,
        products: Vec<Product>,
    },
    NewNotification {
        user_id: u32,
        customer: String,
        success: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseFromRepository {
    Notifications(Vec<Notification>),
    Subscription {
        user_id: u32,
        customer: String,
        product: String,
    },
}

impl ToString for ClientResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for CustomerResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for ClientResponseFromRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for CustomerResponseFromRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for ResponseFromRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
