use std::sync::Arc;

use dotenv::dotenv;
use rabbitmq_client::RabbitMqManager;
use repository::{Config, MessageHandler, RepositoryService, SqliteRepository};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    let mut manager = RabbitMqManager::builder()
        .build(&config.amqp_address)
        .await
        .unwrap();
    let publisher = manager.get_publisher().await.unwrap();

    let repository = SqliteRepository::new(&config.repository_database_url)
        .await
        .unwrap();
    let service = Arc::new(Mutex::new(RepositoryService::new(
        config.clone(),
        repository,
        publisher,
    )));

    manager
        .add_consumer(
            &config.client_repository_request_queue,
            MessageHandler::client_request(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.customer_repository_request_queue,
            MessageHandler::customer_request(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.repository_request_queue,
            MessageHandler::request_to_repository(service.clone()),
        )
        .await
        .unwrap();

    manager.run();
}
