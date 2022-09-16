use std::sync::Arc;

use domain::{requests::RepositoryRequest, responses::RepositoryResponse};
use message_queue::Publisher;
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
        let response_queue = std::env::var("REPOSITORY_RESPONSE_QUEUE").unwrap();
        let config = Config {exchange, response_queue};

        Self { config, repository, publisher }
    }
    pub async fn handle_request(&mut self, request: RepositoryRequest) {

        let response = match request{
            RepositoryRequest::Customers { user_id } => {
                let customers = self.repository.get_customers().await.unwrap();
                RepositoryResponse::Customers{user_id, customers}
            },
            RepositoryRequest::Products { user_id, customer_id } => {
                let products = self.repository.get_products(customer_id).await.unwrap();  
                RepositoryResponse::Products { user_id, products }
            },
            RepositoryRequest::NewSubscription { user_id, customer_id, product_id } => {
                let success = self.repository.add_subscription(user_id, customer_id, product_id).await.is_ok();
                RepositoryResponse::NewSubscription { user_id, success } 
            },
            RepositoryRequest::CustomerAuthorization { user_id, key } => {
                let customer = self.repository.try_authorize(user_id, key).await.unwrap();
                RepositoryResponse::CustomerAuthorization{ user_id, customer }
            },
            RepositoryRequest::Subscriptions { user_id, customer_id } => {
                let products = self.repository.get_subscriptions(customer_id).await.unwrap(); 
                RepositoryResponse::Subscriptions{user_id, products}     
            },
            RepositoryRequest::NewNotification { user_id, customer_id, product_id, notification } => {
                let notifications = self.repository.add_notification(customer_id, product_id, notification).await.unwrap();
                RepositoryResponse::NewNotification { user_id, notifications }
            },

        };
        
        let mut publisher = self.publisher.lock().await;
        publisher
            .publish_message(
                &self.config.exchange,
                &self.config.response_queue,
                serde_json::to_string(&response).unwrap().clone(),
            )
            .await;
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    exchange: String,
    response_queue: String,
}