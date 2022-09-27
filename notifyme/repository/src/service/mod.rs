use domain::{
    requests::{ClientRequestToRepository, CustomerRequestToRepository, RequestToRepository},
    responses::{
        ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository,
    },
};
use rabbitmq_client::Publisher;

use crate::{Config, SqliteRepository};

pub struct RepositoryService {
    config: Config,
    repository: SqliteRepository,
    publisher: Publisher,
}

impl RepositoryService {
    pub fn new(config: Config, repository: SqliteRepository, publisher: Publisher) -> Self {
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

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.client_repository_response_queue,
                response.to_string(),
            )
            .await
            .unwrap();
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

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.customer_repository_response_queue,
                response.to_string(),
            )
            .await
            .unwrap();
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

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.repository_response_queue,
                response.to_string(),
            )
            .await
            .unwrap();
    }
}
