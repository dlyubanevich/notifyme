use serde::{Deserialize, Serialize};

use crate::models::{Customer, Product};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Customers {
        user_id: u32,
    },
    Products {
        user_id: u32,
        customer: Customer,
    },
    NewSubscription {
        user_id: u32,
        customer: Customer,
        product: Product,
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

impl TryFrom<&Request> for RepositoryRequest {
    type Error = ();

    fn try_from(value: &Request) -> Result<Self, Self::Error> {
        match value {
            Request::Customers { user_id } => {
                let user_id = *user_id;
                Ok(RepositoryRequest::Customers { user_id })
            }
            Request::Products { user_id, customer } => {
                let user_id = *user_id;
                let customer_id = customer.id;
                Ok(RepositoryRequest::Products {
                    user_id,
                    customer_id,
                })
            }
            Request::NewSubscription {
                user_id,
                customer,
                product,
            } => {
                let user_id = *user_id;
                let customer_id = customer.id;
                let product_id = product.id;
                Ok(RepositoryRequest::NewSubscription {
                    user_id,
                    customer_id,
                    product_id,
                })
            }
            Request::CustomerAuthorization { user_id, key } => {
                let user_id = *user_id;
                let key = key.to_string();
                Ok(RepositoryRequest::CustomerAuthorization { user_id, key })
            }
            Request::Subscriptions { user_id, customer } => {
                let user_id = *user_id;
                let customer_id = customer.id;
                Ok(RepositoryRequest::Subscriptions {
                    user_id,
                    customer_id,
                })
            }
            Request::NewNotification {
                user_id,
                customer,
                product,
                notification,
            } => {
                let user_id = *user_id;
                let customer_id = customer.id;
                let product_id = product.id;
                let notification = notification.to_string();
                Ok(RepositoryRequest::NewNotification {
                    user_id,
                    customer_id,
                    product_id,
                    notification,
                })
            }
        }
    }
}
