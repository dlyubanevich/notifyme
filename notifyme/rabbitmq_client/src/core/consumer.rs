use std::sync::Arc;

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Channel, ConsumerDelegate,
};
use tokio::sync::Mutex;

use crate::core::IncomingMessageHandler;

#[allow(dead_code)]
pub struct Consumer {
    channel: Channel,
    consumer: lapin::Consumer,
}

impl Consumer {
    pub async fn new(
        channel: Channel,
        queue: &str,
        delegate: impl ConsumerDelegate + 'static,
    ) -> Self {
        let consumer = channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        consumer.set_delegate(delegate);
        Consumer {
            channel,
            consumer,
        }
    }
    pub async fn new2(
        channel: Channel,
        queue: &str,
        handler: Arc<Mutex<dyn IncomingMessageHandler>>,
    ) -> Self {
        let consumer = channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        consumer.set_delegate(Self::delegate(handler));
        Consumer {
            channel,
            consumer,
        }
    }

    fn delegate(
        handler: Arc<Mutex<dyn IncomingMessageHandler>>,
    ) -> impl ConsumerDelegate + 'static {
        move |delivery: DeliveryResult| {
            let handler = handler.clone();
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
                let result = String::from_utf8_lossy(&delivery.data).to_string();
                handler.lock().await.handle_message(result).await;

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack send_webhook_event message");
            }
        }
    }
}
