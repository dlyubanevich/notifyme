use std::sync::Arc;

use domain::{requests::Request, responses::RepositoryResponse};
use message_queue::Publisher;
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
        let repository_request_queue = std::env::var("REPOSITORY_REQUEST_QUEUE").unwrap();
        let history_queue = std::env::var("HISTORY_QUEUE").unwrap();
        let response_queue = std::env::var("RESPONSE_QUEUE").unwrap();
        let config = Config {
            exchange,
            repository_request_queue,
            history_queue,
            response_queue,
        };

        Self { config, publisher }
    }
    pub async fn handle_request(&mut self, request: Request) {
        let record = transform::request_to_record(&request);
        let repository_request = transform::request_to_repository_request(&request);

        let mut publisher = self.publisher.lock().await;
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
                repository_request.to_string(),
            )
            .await;
    }

    pub async fn handle_repository_response(&mut self, repository_response: RepositoryResponse) {
        let record = transform::repository_response_to_record(&repository_response);
        let response = transform::repository_response_to_response(&repository_response);

        let mut publisher = self.publisher.lock().await;
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
                &self.config.response_queue,
                response.to_string(),
            )
            .await;
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    exchange: String,
    repository_request_queue: String,
    history_queue: String,
    response_queue: String,
}
