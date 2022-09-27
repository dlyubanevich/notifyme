use crate::RepositoryService;
use domain::requests::{
    ClientRequestToRepository, CustomerRequestToRepository, RequestToRepository,
};
use rabbitmq_client::IncomingMessageHandler;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MessageHandler {}
impl MessageHandler {
    pub fn client_request(
        service: Arc<Mutex<RepositoryService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let request: ClientRequestToRepository = serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_client_request_to_repository(request)
                    .await;
            }
        }
    }
    pub fn customer_request(
        service: Arc<Mutex<RepositoryService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let request: CustomerRequestToRepository = serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_customer_request_to_repository(request)
                    .await;
            }
        }
    }
    pub fn request_to_repository(
        service: Arc<Mutex<RepositoryService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let request: RequestToRepository = serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_request_to_repository(request)
                    .await;
            }
        }
    }
}
