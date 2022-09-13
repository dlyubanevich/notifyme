use std::future::Future;

use lapin::{options::{BasicConsumeOptions, BasicAckOptions}, types::FieldTable, Channel, ConsumerDelegate, message::DeliveryResult};

pub struct Consumer {
    channel: Channel,
    cons: lapin::Consumer,
}

impl Consumer {
    // pub fn builder() -> ConsumerBuilder {
    //     ConsumerBuilder {}
    // }

    pub async fn new(
        channel: Channel,
        queue: &str,
        delegate: impl ConsumerDelegate + 'static,
    ) -> Self {
        let consumer = channel
            .basic_consume(
                queue,
                "tag_foo",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        consumer.set_delegate(delegate);
        Consumer {
            channel,
            cons: consumer,
        }
    }

    // async fn test_delegate<F> (channel: Channel, queue: &str, func: F) -> impl ConsumerDelegate + 'static 
    // where F: Future<Output = ()> + Send + Sync +'static {

    //      move |delivery: DeliveryResult| {

            
            
    //         async move {
    //             let delivery = match delivery {
    //                 // Carries the delivery alongside its channel
    //                 Ok(Some(delivery)) => delivery,
    //                 // The consumer got canceled
    //                 Ok(None) => return,
    //                 // Carries the error and is always followed by Ok(None)
    //                 Err(error) => {
    //                     dbg!("Failed to consume queue message {}", error);
    //                     return;
    //                 }
    //             };
    
    //             // Do something with the delivery data (The message payload)
    //             let result = String::from_utf8_lossy(&delivery.data).to_string();
    //             func.await;
    
    //             delivery
    //                 .ack(BasicAckOptions::default())
    //                 .await
    //                 .expect("Failed to ack send_webhook_event message");
    //         }
    //     }
    // }
}

// pub struct ConsumerBuilder {
//     //queue: &'static str,  

// }