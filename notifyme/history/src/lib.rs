use std::sync::Arc;

use lapin::{ConsumerDelegate, message::DeliveryResult, options::BasicAckOptions};
use tokio::sync::Mutex;

pub fn request_delegate(message_handler: Arc<Mutex<MessageHandler>>) -> impl ConsumerDelegate + 'static{

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
            handler.lock().await.handle_message(message);

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn response_delegate(message_handler: Arc<Mutex<MessageHandler>>) -> impl ConsumerDelegate + 'static{

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
            handler.lock().await.handle_message(message);

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub fn error_delegate(message_handler: Arc<Mutex<MessageHandler>>) -> impl ConsumerDelegate + 'static{

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
            handler.lock().await.handle_message(message);

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    }
}

pub struct MessageHandler {
    repository: SqliteRepository,
}

pub struct SqliteRepository {
   
    
}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {
            repository: SqliteRepository {}   
        }
    }
    pub fn handle_message(&mut self, message: String) {
        println!("message: [{}]", message);
    }
}