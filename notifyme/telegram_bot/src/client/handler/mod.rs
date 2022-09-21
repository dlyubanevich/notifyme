use std::sync::Arc;

use domain::responses::Response;
use lapin::{message::DeliveryResult, options::BasicAckOptions, ConsumerDelegate};
use tokio::sync::Mutex;

use crate::client::Service;

pub struct MessageHandler {
    service: Service,
}

impl MessageHandler {
    pub fn new(service: Service) -> Self {
        MessageHandler { service }
    }
    pub async fn handle_response(&mut self, message: String) {
        let response: Response = serde_json::from_str(&message).unwrap();
        self.service.handle_response(response).await;
    }
}
pub fn response_delegate(
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
            handler.lock().await.handle_response(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}
