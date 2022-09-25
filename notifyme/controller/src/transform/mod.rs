use std::time::{SystemTime, UNIX_EPOCH};

use domain::{
    models::{CustomerEventRecord, UserEventRecord, UserId, Notification},
    records::Record,
    requests::{ClientRequestToRepository, ClientRequest, CustomerRequest, RequestToRepository, CustomerRequestToRepository},
    responses::{ClientResponseFromRepository, CustomerResponseFromRepository, ClientResponse, CustomerResponse, ResponseFromRepository},
};

pub fn client_request_to_record(request: &ClientRequest) -> Record {

    let data = request.to_string();

    match request {
        ClientRequest::Customers { user_id, timestamp } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let event = "Request for customers".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        ClientRequest::Products { user_id, timestamp, .. } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let event = "Request for products".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
        ClientRequest::NewSubscription { user_id, timestamp, .. } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let event = "Request for new subscription".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        }
    }
}

pub fn client_response_from_repository_to_record(response: &ClientResponseFromRepository) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let data = response.to_string();

    match response {
        ClientResponseFromRepository::Customers { user_id, .. } => {
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
        ClientResponseFromRepository::Products { user_id, .. } => {
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
        ClientResponseFromRepository::NewSubscription { user_id, .. } => {
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
    }
}

pub fn customer_request_to_record(request: &CustomerRequest) -> Record {

    let data = request.to_string();

    match request {
        CustomerRequest::Authorization { user_id, timestamp, .. } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let event = "Request for customer authorization".to_string();
            let customer = None;
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        },
        CustomerRequest::ProductsForNotification { user_id, customer, timestamp } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let customer = Some(customer.clone());
            let event = "Request for products for notification".to_string();  
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        },
        CustomerRequest::NewNotification { user_id, customer, timestamp, .. } => {
            let user_id = user_id.0;
            let timestamp = *timestamp;
            let customer = Some(customer.clone());
            let event = "Request for new notification".to_string();
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        },
    }
}

pub fn customer_response_from_repository_to_record(response: &CustomerResponseFromRepository) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let data = response.to_string();

    match response {
        CustomerResponseFromRepository::Authorization { user_id, customer, .. } => {
            let user_id = *user_id;
            let event = "Response for customer authorization".to_string();
            let customer = customer.clone().map(|customer| customer.name);
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        CustomerResponseFromRepository::ProductsForNotification { user_id, customer, .. } => {
            let user_id = *user_id;
            let event = "Response for products for notification".to_string();
            let customer = Some(customer.clone());
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
        CustomerResponseFromRepository::NewNotification { user_id, customer, .. } => {
            let user_id = *user_id;
            let event = "Response for new notification".to_string();
            let customer = Some(customer.clone());
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        }
    }
}

pub fn request_to_repository_to_record(request: &RequestToRepository) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let data = request.to_string();

    match request {
        RequestToRepository::NotificationForClients { user_id, customer, .. } => {
            let user_id = *user_id;
            let event = "Request for notification for clients".to_string();
            let customer = Some(customer.clone());
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        },
        RequestToRepository::SubscriptionForCustomer { user_id, .. } => {
            let user_id = *user_id;
            let event = "Request for subscription for customer".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        },
    }

}

pub fn response_from_repository_to_record(response: &ResponseFromRepository) -> Record {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let data = response.to_string();

    match response {
        ResponseFromRepository::Notifications(_) => {
            let user_id = 0;
            let event = "Response for notifications for clients".to_string();
            let record = UserEventRecord {
                timestamp,
                user_id,
                data,
                event,
            };
            Record::UserEvent(record)
        },
        ResponseFromRepository::Subscription { user_id, customer, .. } => {
            let user_id = *user_id;
            let event = "Response for subscription for customer".to_string();
            let customer = Some(customer.clone());
            let record = CustomerEventRecord {
                timestamp,
                user_id,
                customer,
                data,
                event,
            };
            Record::CustomerEvent(record)
        },
    }
}

pub fn client_request_to_repository_to_client_request(request: &ClientRequest) -> ClientRequestToRepository {
    match request {
        ClientRequest::Customers { user_id, .. } => {
            let user_id = user_id.0;
            ClientRequestToRepository::Customers { user_id }
        }
        ClientRequest::Products { user_id, customer, .. } => {
            let user_id = user_id.0;
            let customer = customer.clone();
            ClientRequestToRepository::Products {
                user_id,
                customer,
            }
        }
        ClientRequest::NewSubscription { user_id, customer, product, .. } => {
            let user_id = user_id.0;
            let customer = customer.clone();
            let product = product.clone();
            ClientRequestToRepository::NewSubscription {
                user_id,
                customer,
                product,
            }
        }
    }
}

pub fn client_response_from_repository_to_client_response(response: &ClientResponseFromRepository) -> ClientResponse {
    match response {
        ClientResponseFromRepository::Customers { user_id, customers } => {
            let user_id = UserId::from(*user_id);
            let customers = customers.clone();
            ClientResponse::Customers { user_id, customers }
        }
        ClientResponseFromRepository::Products { user_id, products } => {
            let user_id = UserId::from(*user_id);
            let products = products.clone();
            ClientResponse::Products { user_id, products }
        }
        ClientResponseFromRepository::NewSubscription { user_id, success } => {
            let user_id = UserId::from(*user_id);
            match *success {
                true => ClientResponse::SubscriptionSuccess { user_id },
                false => ClientResponse::SubscriptionFailure { user_id },
            }
        }
    }
}

pub fn customer_request_to_repository_to_customer_request(request: &CustomerRequest) -> CustomerRequestToRepository {
    match request {
        CustomerRequest::Authorization { user_id, key, .. } => {
            let user_id = user_id.0;
            let key = key.clone();
            CustomerRequestToRepository::Authorization { user_id, key }
        }
        CustomerRequest::ProductsForNotification { user_id, customer, .. } => {
            let user_id = user_id.0;
            let customer = customer.clone();
            CustomerRequestToRepository::ProductsForNotification {
                user_id,
                customer,
            }
        }
        CustomerRequest::NewNotification {
            user_id,
            customer,
            product,
            notification, ..
        } => {
            let user_id = user_id.0;
            let customer = customer.clone();
            let product = product.clone();
            let notification = notification.clone();
            CustomerRequestToRepository::NewNotification {
                user_id,
                customer,
                product,
                notification,
            }
        }
    }
}

pub fn customer_response_from_repository_to_customer_response(response: &CustomerResponseFromRepository) -> CustomerResponse {
    match response {
        CustomerResponseFromRepository::Authorization { user_id, customer } => {
            let user_id = UserId::from(*user_id);
            match customer {
                Some(customer) => {
                    let customer = customer.clone();
                    CustomerResponse::AuthorizationSuccess { user_id, customer }
                }
                None => CustomerResponse::AuthorizationFailure { user_id },
            }
        }
        CustomerResponseFromRepository::ProductsForNotification {
            user_id, products, customer
        } => {
            let user_id = UserId::from(*user_id);
            let customer = customer.to_owned();
            let products = products.to_owned();
            CustomerResponse::ProductsForNotification { user_id, products, customer }
        }
        CustomerResponseFromRepository::NewNotification {
            user_id,
            success,
            ..
        } => {
            let user_id = UserId::from(*user_id);
            match success {
                true => CustomerResponse::NotificationSuccess { user_id },
                false => CustomerResponse::NotificationFailure { user_id },
            }
        }
    }
}

pub fn client_request_to_repository_request(request: &ClientRequest) -> Option<RequestToRepository> {
    match request {
        ClientRequest::NewSubscription { user_id, customer, product, .. } => {
            let user_id = user_id.0;
            let customer = customer.clone();
            let product = product.clone();
            Some(RequestToRepository::SubscriptionForCustomer { user_id, customer, product })
        },
        _ => None
    }
}

pub fn customer_request_to_repository_request(request: &CustomerRequest) -> Option<RequestToRepository> {
    match request {
        CustomerRequest::NewNotification { user_id, customer, product, notification, .. }=> {
            let user_id = user_id.0;
            let customer = customer.clone();
            let product = product.clone();
            let notification = notification.clone();
            Some(RequestToRepository::NotificationForClients { user_id, customer, product, notification } )
        },
        _ => None
    }
}

pub fn notification_to_client_response(notification: &Notification) -> ClientResponse {
    let user_id = UserId::from(notification.user_id);
    let customer = notification.customer.clone();
    let product = notification.product.clone();
    let notification = notification.text.clone();

    ClientResponse::CustomerNotification { user_id, customer, product, notification }
}
