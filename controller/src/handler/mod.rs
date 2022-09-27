use std::sync::Arc;

use domain::{
    requests::{ClientRequest, CustomerRequest},
    responses::{
        ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository,
    },
};
use amqp::IncomingMessageHandler;
use tokio::sync::Mutex;

use crate::ControllerService;

pub struct MessageHandler {}
impl MessageHandler {
    pub fn client_request(
        service: Arc<Mutex<ControllerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let request: ClientRequest = serde_json::from_str(&message).unwrap();
                service.lock().await.handle_client_request(request).await;
            }
        }
    }

    pub fn customer_request(
        service: Arc<Mutex<ControllerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let request: CustomerRequest = serde_json::from_str(&message).unwrap();
                service.lock().await.handle_customer_request(request).await;
            }
        }
    }

    pub fn client_response_from_repository(
        service: Arc<Mutex<ControllerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let response: ClientResponseFromRepository =
                    serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_client_response_from_repository(response)
                    .await;
            }
        }
    }

    pub fn customer_response_from_repository(
        service: Arc<Mutex<ControllerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let response: CustomerResponseFromRepository =
                    serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_customer_response_from_repository(response)
                    .await;
            }
        }
    }

    pub fn response_from_repository(
        service: Arc<Mutex<ControllerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let response: ResponseFromRepository = serde_json::from_str(&message).unwrap();
                service
                    .lock()
                    .await
                    .handle_response_from_repository(response)
                    .await;
            }
        }
    }
}
