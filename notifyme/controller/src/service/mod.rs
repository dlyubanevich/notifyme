use std::{convert::TryFrom, sync::Arc};

use domain::{
    records::Record,
    requests::{RepositoryRequest, Request},
    responses::{RepositoryResponse, Response},
};
use message_queue::Publisher;
use serde::Deserialize;
use tokio::sync::Mutex;

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
        let config = Config {exchange, repository_request_queue, history_queue, response_queue};

        Self { config, publisher }
    }
    pub async fn handle_request(&mut self, request: Request) {
        let record = Record::try_from(&request).unwrap();
        let repository_request = RepositoryRequest::try_from(&request).unwrap();

        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                serde_json::to_string(&record).unwrap(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.repository_request_queue,
                serde_json::to_string(&repository_request).unwrap().clone(),
            )
            .await;
    }

    pub async fn handle_repository_response(&mut self, response: RepositoryResponse) {
        let record = Record::try_from(&response).unwrap();
        let response = Response::try_from(&response).unwrap();

        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.history_queue,
                serde_json::to_string(&record).unwrap(),
            )
            .await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.response_queue,
                serde_json::to_string(&response).unwrap(),
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
