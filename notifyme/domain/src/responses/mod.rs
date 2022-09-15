use serde::{Deserialize, Serialize};

use crate::models::{Customer, Product};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Customers {
        user_id: u32,
        customers: Vec<Customer>,
    },
    Products {
        user_id: u32,
        products: Vec<Product>,
    },
    SubscriptionSuccess {
        user_id: u32,
    },
    SubscriptionFailure {
        user_id: u32,
    },
    CustomerAuthorizationSuccess {
        user_id: u32,
    },
    CustomerAuthorizationFailure {
        user_id: u32,
    },
    Subscriptions {
        user_id: u32,
        products: Vec<Product>,
    },
    NewNotification {
        user_id: u32,
        product: Product,
        notification: String,
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
        success: bool,
    },
    Subscriptions {
        user_id: u32,
        customer: Customer,
        products: Vec<Product>,
    },
    NewNotification {
        user_id: u32,
        customer: Customer,
        product: Product,
        notification: String,
    },
}

impl TryFrom<&RepositoryResponse> for Response {
    type Error = ();

    fn try_from(value: &RepositoryResponse) -> Result<Self, Self::Error> {
        match value {
            RepositoryResponse::Customers { user_id, customers } => {
                let user_id = *user_id;
                let customers = customers.clone();
                Ok(Response::Customers { user_id, customers })
            }
            RepositoryResponse::Products { user_id, products } => {
                let user_id = *user_id;
                let products = products.clone();
                Ok(Response::Products { user_id, products })
            }
            RepositoryResponse::NewSubscription { user_id, success } => {
                let user_id = *user_id;
                match *success {
                    true => Ok(Response::SubscriptionSuccess { user_id }),
                    false => Ok(Response::SubscriptionFailure { user_id }),
                }
            }
            RepositoryResponse::CustomerAuthorization { user_id, success } => {
                let user_id = *user_id;
                match *success {
                    true => Ok(Response::CustomerAuthorizationSuccess { user_id }),
                    false => Ok(Response::CustomerAuthorizationFailure { user_id }),
                }
            }
            RepositoryResponse::Subscriptions {
                user_id, products, ..
            } => {
                let user_id = *user_id;
                let products = products.clone();
                Ok(Response::Subscriptions { user_id, products })
            }
            RepositoryResponse::NewNotification {
                user_id,
                product,
                notification,
                ..
            } => {
                let user_id = *user_id;
                let product = product.clone();
                let notification = notification.to_string();
                Ok(Response::NewNotification {
                    user_id,
                    product,
                    notification,
                })
            }
        }
    }
}
