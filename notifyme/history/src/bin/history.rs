use dotenv::dotenv;
use history::{repository::SqliteRepository, Config, HistoryService, MessageHandler};
use rabbitmq_client::RabbitMqManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    let repository = SqliteRepository::new(&config.history_database_url)
        .await
        .unwrap();
    let service = Arc::new(Mutex::new(HistoryService::new(repository)));

    let mut client = RabbitMqManager::builder()
        .build(&config.amqp_address)
        .await
        .unwrap();

    client
        .add_consumer(&config.history_queue, MessageHandler::record(service))
        .await
        .unwrap();

    client.run();
}
