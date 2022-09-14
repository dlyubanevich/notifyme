use std::sync::Arc;
use dotenv::dotenv;

use history::{repository::SqliteRepository, service::HistoryService, handler::MessageHandler};
use history::handler::delegate;
use message_queue::Client;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap();
    let queue = std::env::var("QUEUE").unwrap();

    let repository = SqliteRepository::new(&url).await.unwrap();
    let service = HistoryService::new(repository);
    let message_handler = Arc::new(Mutex::new(MessageHandler::new(service)));
    
    let mut client = Client::new("amqp://localhost:5672").await;
    client.with_consumer(&queue, delegate(message_handler.clone())).await;

    client.run();

}