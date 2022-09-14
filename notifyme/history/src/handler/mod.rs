use std::sync::Arc;

use domain::records::Record;
use lapin::{ConsumerDelegate, message::DeliveryResult, options::BasicAckOptions};
use tokio::sync::Mutex;

use crate::service::HistoryService;

pub struct MessageHandler {
    service: HistoryService,
}
impl MessageHandler{
    pub fn new(service: HistoryService) -> Self {
        MessageHandler { service }  
    }
    pub async fn handle_message(&mut self, message: String) {
        let record: Record = serde_json::from_str(&message).unwrap();
        self.service.add_record(record).await;
    }
}

pub fn delegate(message_handler: Arc<Mutex<MessageHandler>>) -> impl ConsumerDelegate + 'static{

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
            handler.lock().await.handle_message(message).await;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}