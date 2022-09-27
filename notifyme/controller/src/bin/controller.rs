use controller::{Config, ControllerService, MessageHandler};
use dotenv::dotenv;
use rabbitmq_client::RabbitMqManager;
use std::sync::Arc;
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
    let service = Arc::new(Mutex::new(ControllerService::new(
        config.clone(),
        publisher,
    )));

    manager
        .add_consumer(
            &config.client_request_queue,
            MessageHandler::client_request(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.customer_request_queue,
            MessageHandler::customer_request(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.client_repository_response_queue,
            MessageHandler::client_response_from_repository(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.customer_repository_response_queue,
            MessageHandler::customer_response_from_repository(service.clone()),
        )
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.repository_response_queue,
            MessageHandler::response_from_repository(service.clone()),
        )
        .await
        .unwrap();

    manager.run();
}
