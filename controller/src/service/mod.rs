use domain::{
    models::UserId,
    requests::{ClientRequest, CustomerRequest},
    responses::{
        ClientResponseFromRepository, CustomerResponse, CustomerResponseFromRepository,
        ResponseFromRepository,
    },
};
use amqp::Publisher;

use crate::{Config, Transformer};

pub struct ControllerService {
    config: Config,
    publisher: Publisher,
}

impl ControllerService {
    pub fn new(config: Config, publisher: Publisher) -> Self {
        Self { config, publisher }
    }

    pub async fn handle_client_request(&mut self, request: ClientRequest) {
        let record = Transformer::client_request_to_record(&request);
        let repository_request =
            Transformer::client_request_to_repository_to_client_request(&request);

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await
            .unwrap();
        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.client_repository_request_queue,
                repository_request.to_string(),
            )
            .await
            .unwrap();

        if let Some(request_to_repository) =
            Transformer::client_request_to_repository_request(&request)
        {
            let record = Transformer::request_to_repository_to_record(&request_to_repository);
            self.publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.history_queue,
                    record.to_string(),
                )
                .await
                .unwrap();
            self.publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.repository_request_queue,
                    request_to_repository.to_string(),
                )
                .await
                .unwrap();
        }
    }

    pub async fn handle_customer_request(&mut self, request: CustomerRequest) {
        let record = Transformer::customer_request_to_record(&request);
        let repository_request =
            Transformer::customer_request_to_repository_to_customer_request(&request);

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await
            .unwrap();
        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.customer_repository_request_queue,
                repository_request.to_string(),
            )
            .await
            .unwrap();

        if let Some(request_to_repository) =
            Transformer::customer_request_to_repository_request(&request)
        {
            let record = Transformer::request_to_repository_to_record(&request_to_repository);
            self.publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.history_queue,
                    record.to_string(),
                )
                .await
                .unwrap();
            self.publisher
                .publish_message(
                    &self.config.exchange,
                    &self.config.repository_request_queue,
                    request_to_repository.to_string(),
                )
                .await
                .unwrap();
        }
    }

    pub async fn handle_client_response_from_repository(
        &mut self,
        repository_response: ClientResponseFromRepository,
    ) {
        let record = Transformer::client_response_from_repository_to_record(&repository_response);
        let response =
            Transformer::client_response_from_repository_to_client_response(&repository_response);

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await
            .unwrap();
        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.client_response_queue,
                response.to_string(),
            )
            .await
            .unwrap();
    }

    pub async fn handle_customer_response_from_repository(
        &mut self,
        repository_response: CustomerResponseFromRepository,
    ) {
        let record = Transformer::customer_response_from_repository_to_record(&repository_response);
        let response = Transformer::customer_response_from_repository_to_customer_response(
            &repository_response,
        );

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await
            .unwrap();
        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.customer_response_queue,
                response.to_string(),
            )
            .await
            .unwrap();
    }

    pub async fn handle_response_from_repository(
        &mut self,
        repository_response: ResponseFromRepository,
    ) {
        let record = Transformer::response_from_repository_to_record(&repository_response);

        self.publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                record.to_string(),
            )
            .await
            .unwrap();

        match repository_response {
            ResponseFromRepository::Notifications(notifications) => {
                for notification in notifications {
                    let response = Transformer::notification_to_client_response(&notification);
                    self.publisher
                        .publish_message(
                            &self.config.exchange,
                            &self.config.client_response_queue,
                            response.to_string(),
                        )
                        .await
                        .unwrap();
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
                self.publisher
                    .publish_message(
                        &self.config.exchange,
                        &self.config.customer_response_queue,
                        response.to_string(),
                    )
                    .await
                    .unwrap();
            }
        }
    }
}
