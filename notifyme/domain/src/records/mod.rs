use std::{
    convert::TryFrom,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{
    models::{CustomerEventRecord, UserEventRecord},
    requests::Request,
    responses::RepositoryResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Record {
    UserEvent(UserEventRecord),
    CustomerEvent(CustomerEventRecord),
}

impl TryFrom<&Request> for Record {
    type Error = ();

    fn try_from(value: &Request) -> Result<Self, Self::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let data = serde_json::to_string(&value).unwrap();

        match value {
            Request::Customers { user_id } => {
                let user_id = *user_id;
                let event = "Request for customers".to_string();
                let record = UserEventRecord {
                    timestamp,
                    user_id,
                    data,
                    event,
                };
                Ok(Record::UserEvent(record))
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
                Ok(Record::UserEvent(record))
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
                Ok(Record::UserEvent(record))
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
                Ok(Record::CustomerEvent(record))
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
                Ok(Record::CustomerEvent(record))
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
                Ok(Record::CustomerEvent(record))
            }
        }
    }
}

impl TryFrom<&RepositoryResponse> for Record {
    type Error = ();

    fn try_from(value: &RepositoryResponse) -> Result<Self, Self::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let data = serde_json::to_string(&value).unwrap();

        match value {
            RepositoryResponse::Customers { user_id, .. } => {
                let user_id = *user_id;
                let event = "Response for customers".to_string();
                let record = UserEventRecord {
                    timestamp,
                    user_id,
                    data,
                    event,
                };
                Ok(Record::UserEvent(record))
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
                Ok(Record::UserEvent(record))
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
                Ok(Record::UserEvent(record))
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
                Ok(Record::CustomerEvent(record))
            }
            RepositoryResponse::Subscriptions {
                user_id, customer, ..
            } => {
                let user_id = *user_id;
                let event = "Response for subscriptions".to_string();
                let customer_id = customer.id;
                let record = CustomerEventRecord {
                    timestamp,
                    user_id,
                    customer_id,
                    data,
                    event,
                };
                Ok(Record::CustomerEvent(record))
            }
            RepositoryResponse::NewNotification {
                user_id,
                customer,
                notification,
                ..
            } => {
                let user_id = *user_id;
                let event = "Response for new notification".to_string();
                let customer_id = customer.id;
                let record = CustomerEventRecord {
                    timestamp,
                    user_id,
                    customer_id,
                    data,
                    event,
                };
                Ok(Record::CustomerEvent(record))
            }
        }
    }
}
