use std::sync::Arc;

use domain::{
    requests::{ClientRequest, CustomerRequest},
    responses::{
        ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository,
    },
};
use lapin::{message::DeliveryResult, options::BasicAckOptions, ConsumerDelegate};
use tokio::sync::Mutex;

use crate::ControllerService;

pub struct MessageHandler {
    service: ControllerService,
}

impl MessageHandler {
    pub fn new(service: ControllerService) -> Self {
        MessageHandler { service }
    }
    pub async fn handle_client_request(&mut self, message: String) {
        let request: ClientRequest = serde_json::from_str(&message).unwrap();
        self.service.handle_client_request(request).await;
    }
    pub async fn handle_customer_request(&mut self, message: String) {
        let request: CustomerRequest = serde_json::from_str(&message).unwrap();
        self.service.handle_customer_request(request).await;
    }
    pub async fn handle_client_response_from_repository(&mut self, message: String) {
        let response: ClientResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service
            .handle_client_response_from_repository(response)
            .await;
    }
    pub async fn handle_customer_response_from_repository(&mut self, message: String) {
        let response: CustomerResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service
            .handle_customer_response_from_repository(response)
            .await;
    }
    pub async fn handle_response_from_repository(&mut self, message: String) {
        let response: ResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service.handle_response_from_repository(response).await;
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
            handler.lock().await.handle_client_request(message).await;

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
            handler.lock().await.handle_customer_request(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn client_response_from_repository_delegate(
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
            handler
                .lock()
                .await
                .handle_client_response_from_repository(message)
                .await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn customer_response_from_repository_delegate(
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
            handler
                .lock()
                .await
                .handle_customer_response_from_repository(message)
                .await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn response_from_repository_delegate(
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
            handler
                .lock()
                .await
                .handle_response_from_repository(message)
                .await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}
