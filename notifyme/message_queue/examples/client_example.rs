use lapin::{message::DeliveryResult, options::BasicAckOptions};
use message_queue::Client;


#[tokio::main]
async fn main() {

    let uri = "amqp://localhost:5672";
    let mut client = Client::new(uri).await;
    
    let publisher = client.get_publisher();

    let delegate = move |delivery: DeliveryResult| {

        let publisher = publisher.clone();

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
            
            publisher.lock().await.publish_message("exchange", "rooting_key", result).await;

            //println!("delivery message: [{}]", result);

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        }
    };

    client.with_consumer("test", delegate).await;
    client.run();
} 