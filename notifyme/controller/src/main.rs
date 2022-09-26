use controller::{
    client_request_delegate, client_response_from_repository_delegate, customer_request_delegate,
    customer_response_from_repository_delegate, response_from_repository_delegate,
    ControllerService, MessageHandler,
};
use dotenv::dotenv;
use rabbitmq_client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client_request_queue = std::env::var("CLIENT_REQUEST_QUEUE").unwrap();
    let customer_request_queue = std::env::var("CUSTOMER_REQUEST_QUEUE").unwrap();
    let client_repository_response_queue =
        std::env::var("CLIENT_REPOSITORY_RESPONSE_QUEUE").unwrap();
    let customer_repository_response_queue =
        std::env::var("CUSTOMER_REPOSITORY_RESPONSE_QUEUE").unwrap();
    let repository_response_queue = std::env::var("REPOSITORY_RESPONSE_QUEUE").unwrap();

    let mut client = Client::new("amqp://localhost:5672").await;

    let publisher = client.get_publisher();
    let service = ControllerService::new(publisher);
    let message_handler = Arc::new(Mutex::new(MessageHandler::new(service)));

    client
        .with_consumer(
            &client_request_queue,
            client_request_delegate(message_handler.clone()),
        )
        .await;
    client
        .with_consumer(
            &customer_request_queue,
            customer_request_delegate(message_handler.clone()),
        )
        .await;
    client
        .with_consumer(
            &client_repository_response_queue,
            client_response_from_repository_delegate(message_handler.clone()),
        )
        .await;
    client
        .with_consumer(
            &customer_repository_response_queue,
            customer_response_from_repository_delegate(message_handler.clone()),
        )
        .await;
    client
        .with_consumer(
            &repository_response_queue,
            response_from_repository_delegate(message_handler.clone()),
        )
        .await;

    client.run();
}
