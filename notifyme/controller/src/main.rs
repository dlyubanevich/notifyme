use controller::{
    repository_response_delegate, request_delegate, ControllerService, MessageHandler,
};
use dotenv::dotenv;
use message_queue::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let request_queue = std::env::var("REQUEST_QUEUE").unwrap();
    let repository_response_queue = std::env::var("REPOSITORY_RESPONSE_QUEUE").unwrap();

    let mut client = Client::new("amqp://localhost:5672").await;

    let publisher = client.get_publisher();
    let service = ControllerService::new(publisher);
    let message_handler = Arc::new(Mutex::new(MessageHandler::new(service)));

    client
        .with_consumer(
            &request_queue,
            request_delegate(message_handler.clone()),
        )
        .await;
    client
        .with_consumer(
            &repository_response_queue,
            repository_response_delegate(message_handler.clone()),
        )
        .await;

    client.run();
}
#[derive(Deserialize, Debug)]
struct Config {
    request_queue: String,
    repository_response_queue: String,
}
