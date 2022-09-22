use serde::{Deserialize, Serialize};

use crate::models::UserId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRequest {
    Customers {
        user_id: UserId,
        timestamp: i64,
    },
    Products {
        user_id: UserId,
        customer: String,
        timestamp: i64,
    },
    NewSubscription {
        user_id: UserId,
        customer: String,
        product: String,
        timestamp: i64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomerRequest {
    Authorization {
        user_id: UserId,
        key: String,
        timestamp: i64,
    },
    ProductsForNotification {
        user_id: UserId,
        customer: String,
        timestamp: i64,
    },
    NewNotification {
        user_id: UserId,
        customer: String,
        product: String,
        notification: String,
        timestamp: i64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRequestToRepository {
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomerRequestToRepository {
    Authorization {
        user_id: u32,
        key: String,
    },
    ProductsForNotification {
        user_id: u32,
        customer: String,
    },
    NewNotification {
        user_id: u32,
        customer: String,
        product: String,
        notification: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestToRepository {
    NotificationForClients {
        user_id: u32,
        customer: String,
        product: String,
        notification: String,
    },
    SubscriptionForCustomer {
        user_id: u32,
        customer: String,
        product: String,
    }
}    

impl ToString for ClientRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for CustomerRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for ClientRequestToRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for CustomerRequestToRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for RequestToRepository {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
