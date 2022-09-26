use controller::ControllerService;
use domain::{requests::{ClientRequest, CustomerRequest}, responses::{ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository}};
use dotenv::dotenv;
use rabbitmq_client::{RabbitMqClient, IncomingMessageHandler};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    // TODO Загрузить все в конфиг
    let rabbitmq_uri = std::env::var("RABBITMQ_URI").unwrap();
    let client_request_queue = std::env::var("CLIENT_REQUEST_QUEUE").unwrap();
    let customer_request_queue = std::env::var("CUSTOMER_REQUEST_QUEUE").unwrap();
    let client_repository_response_queue =
        std::env::var("CLIENT_REPOSITORY_RESPONSE_QUEUE").unwrap();
    let customer_repository_response_queue =
        std::env::var("CUSTOMER_REPOSITORY_RESPONSE_QUEUE").unwrap();
    let repository_response_queue = std::env::var("REPOSITORY_RESPONSE_QUEUE").unwrap();

    let mut client = RabbitMqClient::builder()
        .with_publisher()
        .build(&rabbitmq_uri)
        .await
        .unwrap();
    let publisher = client.get_publisher().unwrap();
    let service = Arc::new(Mutex::new(ControllerService::new(publisher)));

    client.add_consumer(&client_request_queue, client_request_handle_messages(service.clone())).await;
    client.add_consumer(&customer_request_queue, customer_request_handle_messages(service.clone())).await;
    client.add_consumer(&client_repository_response_queue, client_repository_response_handle_messages(service.clone())).await;
    client.add_consumer(&customer_repository_response_queue, customer_repository_response_handle_messages(service.clone())).await;
    client.add_consumer(&repository_response_queue, repository_response_handle_messages(service.clone())).await;
        
    client.run();

}

fn client_request_handle_messages(service: Arc<Mutex<ControllerService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let request: ClientRequest = serde_json::from_str(&message).unwrap();
            service.lock().await.handle_client_request(request).await;     
        }
    }
}

fn customer_request_handle_messages(service: Arc<Mutex<ControllerService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let request: CustomerRequest = serde_json::from_str(&message).unwrap();
            service.lock().await.handle_customer_request(request).await;     
        }
    }
}

fn client_repository_response_handle_messages(service: Arc<Mutex<ControllerService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let response: ClientResponseFromRepository = serde_json::from_str(&message).unwrap();
            service.lock().await
            .handle_client_response_from_repository(response)
            .await;     
        }
    }
}

fn customer_repository_response_handle_messages(service: Arc<Mutex<ControllerService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let response: CustomerResponseFromRepository = serde_json::from_str(&message).unwrap();
            service.lock().await
                .handle_customer_response_from_repository(response)
                .await;     
        }
    }
}

fn repository_response_handle_messages(service: Arc<Mutex<ControllerService>>) -> impl IncomingMessageHandler + 'static {
    move |message: String| {
        let service = service.clone();
        async move {
            let response: ResponseFromRepository = serde_json::from_str(&message).unwrap();
            service.lock().await.handle_response_from_repository(response).await;     
        }
    }
}