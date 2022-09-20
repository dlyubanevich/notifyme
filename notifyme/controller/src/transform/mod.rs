use std::time::{SystemTime, UNIX_EPOCH};

use domain::{
    models::{CustomerEventRecord, UserEventRecord},
    records::Record,
    requests::{RepositoryRequest, Request},
    responses::{RepositoryResponse, Response},
};

pub fn request_to_record(request: &Request) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let data = serde_json::to_string(request).unwrap();

    match request {
        Request::Customers { user_id } => {
            let user_id = *user_id;
            let event = "Request for customers".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        Request::Products { user_id, .. } => {
            let user_id = *user_id;
            let event = "Request for products".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        Request::NewSubscription { user_id, .. } => {
            let user_id = *user_id;
            let event = "Request for new subscription".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        Request::CustomerAuthorization { user_id, .. } => {
            let user_id = *user_id;
            let event = "Request for customer authorization".to_string();
            let customer_id = 0;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        Request::Subscriptions { user_id, customer } => {
            let user_id = *user_id;
            let event = "Request for subscriptions".to_string();
            let customer_id = customer.id;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        Request::NewNotification {
            user_id, customer, ..
        } => {
            let user_id = *user_id;
            let event = "Request for new notification".to_string();
            let customer_id = customer.id;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
    }
}
pub fn repository_response_to_record(response: &RepositoryResponse) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let data = serde_json::to_string(&response).unwrap();

    match response {
        RepositoryResponse::Customers { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for customers".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        RepositoryResponse::Products { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for products".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        RepositoryResponse::NewSubscription { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for new subscription".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        RepositoryResponse::CustomerAuthorization { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for customer authorization".to_string();
            let customer_id = 0;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        RepositoryResponse::Subscriptions { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for subscriptions".to_string();
            let customer_id = 0;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        RepositoryResponse::NewNotification { user_id, .. } => {
            let user_id = *user_id;
            let event = "Response for new notification".to_string();
            let customer_id = 0;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer_id,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
    }
}

pub fn request_to_repository_request(request: &Request) -> RepositoryRequest {
    match request {
        Request::Customers { user_id } => {
            let user_id = *user_id;
            RepositoryRequest::Customers { user_id }
        }
        Request::Products { user_id, .. } => {
            let user_id = *user_id;
            let customer_id = 1;
            RepositoryRequest::Products {
                user_id,
                customer_id,
            }
        }
        Request::NewSubscription { user_id, .. } => {
            let user_id = *user_id;
            let customer_id = 1;
            let product_id = 1;
            RepositoryRequest::NewSubscription {
                user_id,
                customer_id,
                product_id,
            }
        }
        Request::CustomerAuthorization { user_id, key } => {
            let user_id = *user_id;
            let key = key.to_string();
            RepositoryRequest::CustomerAuthorization { user_id, key }
        }
        Request::Subscriptions { user_id, customer } => {
            let user_id = *user_id;
            let customer_id = customer.id;
            RepositoryRequest::Subscriptions {
                user_id,
                customer_id,
            }
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
            RepositoryRequest::NewNotification {
                user_id,
                customer_id,
                product_id,
                notification,
            }
        }
    }
}
pub fn repository_response_to_response(response: &RepositoryResponse) -> Response {
    match response {
        RepositoryResponse::Customers { user_id, customers } => {
            let user_id = *user_id;
            let customers = customers
                .iter()
                .map(|customer| customer.name.to_owned())
                .collect();
            Response::Customers { user_id, customers }
        }
        RepositoryResponse::Products { user_id, products } => {
            let user_id = *user_id;
            let products = products
                .iter()
                .map(|product| product.name.to_owned())
                .collect();
            Response::Products { user_id, products }
        }
        RepositoryResponse::NewSubscription { user_id, success } => {
            let user_id = *user_id;
            match *success {
                true => Response::SubscriptionSuccess { user_id },
                false => Response::SubscriptionFailure { user_id },
            }
        }
        RepositoryResponse::CustomerAuthorization { user_id, customer } => {
            let user_id = *user_id;
            match customer {
                Some(customer) => {
                    let customer = customer.clone();
                    Response::CustomerAuthorizationSuccess { user_id, customer }
                }
                None => Response::CustomerAuthorizationFailure { user_id },
            }
        }
        RepositoryResponse::Subscriptions {
            user_id, products, ..
        } => {
            let user_id = *user_id;
            let products = products
                .iter()
                .map(|product| product.name.to_owned())
                .collect();
            Response::Subscriptions { user_id, products }
        }
        RepositoryResponse::NewNotification {
            user_id,
            notifications,
            ..
        } => {
            let user_id = *user_id;
            let notifications = notifications.clone();
            Response::NewNotification {
                user_id,
                notifications,
            }
        }
    }
}
