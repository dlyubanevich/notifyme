use domain::records::Record;
use dotenv::dotenv;
use std::sync::Arc;

use history::{repository::SqliteRepository, service::HistoryService};
use rabbitmq_client::{IncomingMessageHandler, RabbitMqClient};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap();
    let queue = std::env::var("HISTORY_QUEUE").unwrap();

    let repository = SqliteRepository::new(&url).await.unwrap();
    let service = Arc::new(Mutex::new(HistoryService::new(repository)));

    let client = RabbitMqClient::builder()
        .with_consumer(&queue, handle_messages(service))
        .build("amqp://localhost:5672")
        .await
        .unwrap();

    client.run();
}

fn handle_messages(service: Arc<Mutex<HistoryService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let record: Record = serde_json::from_str(&message).unwrap();
            service.lock().await.add_record(record).await;
        }
    }
}
