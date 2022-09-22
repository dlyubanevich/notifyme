use crate::RepositoryService;
use domain::requests::{ClientRequestToRepository, CustomerRequestToRepository, RequestToRepository};
use lapin::{message::DeliveryResult, options::BasicAckOptions, ConsumerDelegate};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MessageHandler {
    service: RepositoryService,
}
impl MessageHandler {
    pub fn new(service: RepositoryService) -> Self {
        MessageHandler { service }
    }
    pub async fn handle_client_request_to_repository(&mut self, message: String) {
        let request: ClientRequestToRepository = serde_json::from_str(&message).unwrap();
        self.service.handle_client_request_to_repository(request).await;
    }
    pub async fn handle_customer_request_to_repository(&mut self, message: String) {
        let request: CustomerRequestToRepository = serde_json::from_str(&message).unwrap();
        self.service.handle_customer_request_to_repository(request).await;
    }
    pub async fn handle_request_to_repository(&mut self, message: String) {
        let request: RequestToRepository = serde_json::from_str(&message).unwrap();
        self.service.handle_request_to_repository(request).await;
    }
}

pub fn client_request_delegate(
    message_handler: Arc<Mutex<MessageHandler>>,
) -> impl ConsumerDelegate + 'static {
    move |delivery: DeliveryResult| {
        let handler = message_handler.clone();

        async move {
            let delivery = match delivery {
                // Carries the delivery alongside its channel
                Ok(Some(delivery)) => delivery,
                // The consumer got canceled
                Ok(None) => return,
                // Carries the error and is always followed by Ok(None)
                Err(error) => {
                    dbg!("Failed to consume queue message {}", error);
                    return;
                }
            };

            // Do something with the delivery data (The message payload)
            let message = String::from_utf8_lossy(&delivery.data).to_string();
            handler.lock().await.handle_client_request_to_repository(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn customer_request_delegate(
    message_handler: Arc<Mutex<MessageHandler>>,
) -> impl ConsumerDelegate + 'static {
    move |delivery: DeliveryResult| {
        let handler = message_handler.clone();

        async move {
            let delivery = match delivery {
                // Carries the delivery alongside its channel
                Ok(Some(delivery)) => delivery,
                // The consumer got canceled
                Ok(None) => return,
                // Carries the error and is always followed by Ok(None)
                Err(error) => {
                    dbg!("Failed to consume queue message {}", error);
                    return;
                }
            };

            // Do something with the delivery data (The message payload)
            let message = String::from_utf8_lossy(&delivery.data).to_string();
            handler.lock().await.handle_customer_request_to_repository(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn request_delegate(
    message_handler: Arc<Mutex<MessageHandler>>,
) -> impl ConsumerDelegate + 'static {
    move |delivery: DeliveryResult| {
        let handler = message_handler.clone();

        async move {
            let delivery = match delivery {
                // Carries the delivery alongside its channel
                Ok(Some(delivery)) => delivery,
                // The consumer got canceled
                Ok(None) => return,
                // Carries the error and is always followed by Ok(None)
                Err(error) => {
                    dbg!("Failed to consume queue message {}", error);
                    return;
                }
            };

            // Do something with the delivery data (The message payload)
            let message = String::from_utf8_lossy(&delivery.data).to_string();
            handler.lock().await.handle_request_to_repository(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}