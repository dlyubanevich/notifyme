use std::sync::Arc;

use domain::{
    models::UserId,
    requests::{ClientRequest, CustomerRequest},
    responses::{
        ClientResponseFromRepository, CustomerResponse, CustomerResponseFromRepository,
        ResponseFromRepository,
    },
};
use rabbitmq_client::Publisher;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::transform;

pub struct ControllerService {
    config: Config,
    publisher: Arc<Mutex<Publisher>>,
}

impl ControllerService {
    pub fn new(publisher: Arc<Mutex<Publisher>>) -> Self {
        let exchange = std::env::var("EXCHANGE").unwrap();
        let repository_client_request_queue =
            std::env::var("CLIENT_REPOSITORY_REQUEST_QUEUE").unwrap();
        let repository_customer_request_queue =
            std::env::var("CUSTOMER_REPOSITORY_REQUEST_QUEUE").unwrap();
        let repository_request_queue = std::env::var("REPOSITORY_REQUEST_QUEUE").unwrap();
        let history_queue = std::env::var("HISTORY_QUEUE").unwrap();
        let client_response_queue = std::env::var("CLIENT_RESPONSE_QUEUE").unwrap();
        let customer_response_queue = std::env::var("CUSTOMER_RESPONSE_QUEUE").unwrap();
        let config = Config {
            exchange,
            repository_client_request_queue,
            repository_customer_request_queue,
            repository_request_queue,
            history_queue,
            client_response_queue,
            customer_response_queue,
        };

        Self { config, publisher }
    }

    pub async fn handle_client_request(&mut self, request: ClientRequest) {
        let record = transform::client_request_to_record(&request);
        let repository_request =
            transform::client_request_to_repository_to_client_request(&request);

        let publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.repository_client_request_queue,
                repository_request.to_string(),
            )
            .await;

        if let Some(request_to_repository) =
            transform::client_request_to_repository_request(&request)
        {
            let record = transform::request_to_repository_to_record(&request_to_repository);
            publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.history_queue,
                    record.to_string(),
                )
                .await;
            publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.repository_request_queue,
                    request_to_repository.to_string(),
                )
                .await;
        }
    }

    pub async fn handle_customer_request(&mut self, request: CustomerRequest) {
        let record = transform::customer_request_to_record(&request);
        let repository_request =
            transform::customer_request_to_repository_to_customer_request(&request);

        let publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.repository_customer_request_queue,
                repository_request.to_string(),
            )
            .await;

        if let Some(request_to_repository) =
            transform::customer_request_to_repository_request(&request)
        {
            let record = transform::request_to_repository_to_record(&request_to_repository);
            publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.history_queue,
                    record.to_string(),
                )
                .await;
            publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.repository_request_queue,
                    request_to_repository.to_string(),
                )
                .await;
        }
    }

    pub async fn handle_client_response_from_repository(
        &mut self,
        repository_response: ClientResponseFromRepository,
    ) {
        let record = transform::client_response_from_repository_to_record(&repository_response);
        let response =
            transform::client_response_from_repository_to_client_response(&repository_response);

        let publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.client_response_queue,
                response.to_string(),
            )
            .await;
    }

    pub async fn handle_customer_response_from_repository(
        &mut self,
        repository_response: CustomerResponseFromRepository,
    ) {
        let record = transform::customer_response_from_repository_to_record(&repository_response);
        let response =
            transform::customer_response_from_repository_to_customer_response(&repository_response);

        let publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.customer_response_queue,
                response.to_string(),
            )
            .await;
    }

    pub async fn handle_response_from_repository(
        &mut self,
        repository_response: ResponseFromRepository,
    ) {
        let record = transform::response_from_repository_to_record(&repository_response);

        let publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await;

        match repository_response {
            ResponseFromRepository::Notifications(notifications) => {
                for notification in notifications {
                    let response = transform::notification_to_client_response(&notification);
                    publisher
                        .publish_message(
                            &self.config.exchange,
                            &self.config.client_response_queue,
                            response.to_string(),
                        )
                        .await;
                }
            }
            ResponseFromRepository::Subscription {
                user_id,
                customer,
                product,
            } => {
                let user_id = UserId::from(user_id);
                let response = CustomerResponse::ClientSubscription {
                    user_id,
                    customer,
                    product,
                };
                publisher
                    .publish_message(
                        &self.config.exchange,
                        &self.config.customer_response_queue,
                        response.to_string(),
                    )
                    .await;
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    exchange: String,
    repository_client_request_queue: String,
    repository_customer_request_queue: String,
    repository_request_queue: String,
    history_queue: String,
    client_response_queue: String,
    customer_response_queue: String,
}
