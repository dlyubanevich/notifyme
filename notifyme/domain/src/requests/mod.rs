use serde::{Serialize, Deserialize};

use crate::models::{Customer, Product};

#[derive(Serialize, Deserialize)]
pub enum Request {
    Customers { user_id: u32 },
    Products { user_id: u32, customer: Customer },
    NewSubscription { user_id: u32, customer: Customer, product: Product },
    CustomerAuthorization { user_id: u32, key: String },
    Subscriptions { user_id: u32, customer: Customer },
    NewNotification { user_id: u32, customer: Customer, product: Product, notification: String },
}