use std::sync::Arc;

use domain::{
    requests::{ClientRequestToRepository, CustomerRequestToRepository, RequestToRepository},
    responses::{
        ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository,
    },
};
use rabbitmq_client::Publisher;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::SqliteRepository;

pub struct RepositoryService {
    config: Config,
    repository: SqliteRepository,
    publisher: Arc<Mutex<Publisher>>,
}

impl RepositoryService {
    pub fn new(repository: SqliteRepository, publisher: Arc<Mutex<Publisher>>) -> Self {
        let exchange = std::env::var("EXCHANGE").unwrap();
        let client_response_queue = std::env::var("CLIENT_REPOSITORY_RESPONSE_QUEUE").unwrap();
        let customer_response_queue = std::env::var("CUSTOMER_REPOSITORY_RESPONSE_QUEUE").unwrap();
        let response_queue = std::env::var("REPOSITORY_RESPONSE_QUEUE").unwrap();
        let config = Config {
            exchange,
            client_response_queue,
            customer_response_queue,
            response_queue,
        };

        Self {
            config,
            repository,
            publisher,
        }
    }
    pub async fn handle_client_request_to_repository(
        &mut self,
        request: ClientRequestToRepository,
    ) {
        let response = match request {
            ClientRequestToRepository::Customers { user_id } => {
                let customers = self.repository.get_customers().await.unwrap();
                ClientResponseFromRepository::Customers { user_id, customers }
            }
            ClientRequestToRepository::Products { user_id, customer } => {
                let products = self.repository.get_products(&customer).await.unwrap();
                ClientResponseFromRepository::Products { user_id, products }
            }
            ClientRequestToRepository::NewSubscription {
                user_id,
                customer,
                product,
            } => {
                let success = self
                    .repository
                    .add_subscription(user_id, &customer, &product)
                    .await
                    .is_ok();
                ClientResponseFromRepository::NewSubscription { user_id, success }
            }
        };

        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.client_response_queue,
                response.to_string(),
            )
            .await;
    }
    pub async fn handle_customer_request_to_repository(
        &mut self,
        request: CustomerRequestToRepository,
    ) {
        let response = match request {
            CustomerRequestToRepository::Authorization { user_id, key } => {
                let customer = self.repository.try_authorize(user_id, key).await.unwrap();
                CustomerResponseFromRepository::Authorization { user_id, customer }
            }
            CustomerRequestToRepository::ProductsForNotification { user_id, customer } => {
                let products = self
                    .repository
                    .get_products_for_notification(&customer)
                    .await
                    .unwrap();
                CustomerResponseFromRepository::ProductsForNotification {
                    user_id,
                    customer,
                    products,
                }
            }
            CustomerRequestToRepository::NewNotification {
                user_id,
                customer,
                product,
                notification,
            } => {
                let success = self
                    .repository
                    .add_notification(&customer, &product, notification)
                    .await
                    .is_ok();
                CustomerResponseFromRepository::NewNotification {
                    user_id,
                    customer,
                    success,
                }
            }
        };

        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.customer_response_queue,
                response.to_string(),
            )
            .await;
    }
    pub async fn handle_request_to_repository(&mut self, request: RequestToRepository) {
        let response = match request {
            RequestToRepository::NotificationForClients {
                customer,
                product,
                notification,
                ..
            } => {
                let notifications = self
                    .repository
                    .get_notifications(&customer, &product, notification)
                    .await
                    .unwrap();
                ResponseFromRepository::Notifications(notifications)
            }
            RequestToRepository::SubscriptionForCustomer {
                customer, product, ..
            } => {
                let user_id = self
                    .repository
                    .get_customers_user_id(&customer)
                    .await
                    .unwrap();
                ResponseFromRepository::Subscription {
                    user_id,
                    customer,
                    product,
                }
            }
        };

        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.response_queue,
                response.to_string(),
            )
            .await;
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    exchange: String,
    client_response_queue: String,
    customer_response_queue: String,
    response_queue: String,
}
