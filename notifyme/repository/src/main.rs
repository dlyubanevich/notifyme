
use dotenv::dotenv;
use message_queue::Client;
use repository::{RepositoryService, MessageHandler, request_delegate, SqliteRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let request_queue = std::env::var("REPOSITORY_REQUEST_QUEUE").unwrap();
    let mut client = Client::new("amqp://localhost:5672").await;

    let repository = SqliteRepository::new("sqlite:repository.db").await.unwrap();
    let publisher = client.get_publisher();
    let service = RepositoryService::new(repository, publisher);
    let message_handler = Arc::new(Mutex::new(MessageHandler::new(service)));

    client
        .with_consumer(
            &request_queue,
            request_delegate(message_handler.clone()),
        )
        .await;

    client.run();
}
