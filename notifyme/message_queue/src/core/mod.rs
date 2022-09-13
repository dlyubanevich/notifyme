use std::sync::Arc;

use lapin::{
    options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties,
    ConsumerDelegate,
};
use tokio::sync::Mutex;

mod consumer;
mod model;
mod publisher;

pub struct Client {
    connection: Connection,
    consumers: Vec<consumer::Consumer>,
    publisher: Arc<Mutex<publisher::Publisher>>,
}

impl Client {
    // pub fn builder() -> ClientBuilder {

    // }

    pub async fn new(uri: &str) -> Self {
        let options = ConnectionProperties::default()
            // Use tokio executor and reactor.
            // At the moment the reactor is only available for unix.
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let connection = Connection::connect(uri, options).await.unwrap();
        let channel = connection.create_channel().await.unwrap();
        let publisher = Arc::new(Mutex::new(publisher::Publisher::new(channel)));

        Client {
            connection,
            consumers: vec![],
            publisher,
        }
    }
    pub fn get_publisher(&self) -> Arc<Mutex<publisher::Publisher>> {
        Arc::clone(&self.publisher) 
    }
    pub async fn send_message(&mut self, exchange: &str, rooting_key: &str, message: String) {
        self.publisher.lock().await
            .publish_message(exchange, rooting_key, message)
            .await;
        //TODO Тут вся проверка отправки сообщения. Если возвращаемое значение не Ок, то нужно хранить эти сообщения где-нибудь для последующей отправки
    }

    pub async fn with_consumer(&mut self, queue: &str, delegate: impl ConsumerDelegate + 'static) {
        let channel = self.connection.create_channel().await.unwrap();
        let consumer = consumer::Consumer::new(channel, queue, delegate).await;
        self.consumers.push(consumer);
    }

    pub fn run(self){
        self.connection.run().expect("connection run");
    }
    //TODO

    // Добавить к консумеру установку делегата на чтение очереди
    // Все это обернуть в билдера
    // Развернуть RabbitMQ в Докере и написать тесты, на отправку и получение сообщений
}

pub struct ClientBuilder {
    uri: &'static str,
    consumers: Vec<consumer::Consumer>,
    
}

impl ClientBuilder {
    
}
#[cfg(test)]
mod tests {
    use lapin::{message::DeliveryResult, options::BasicAckOptions};

    use super::*;

    const URI: &str = "amqp://127.0.0.1:5672";
    const EXCHANGE: &str = "test";
    const ROUTING_KEY: &str = "test";
    const QUEUE: &str = "test";

    #[tokio::test]
    async fn should_send_message() {
        let mut client = Client::new(URI).await;
        let message = "Work bitch".to_string();
        client.send_message(EXCHANGE, ROUTING_KEY, message).await;
    }

    #[tokio::test]
    async fn should_handle_incomming_message() {

        let mut client = Client::new(URI).await;
        let message = "It is done".to_string();

        let string = String::new();

        let delegate = move |delivery: DeliveryResult| {
            let mut string = string.clone();
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
    
                let result = String::from_utf8_lossy(&delivery.data);
                println!("delivery message: [{}]", result);

                string.push_str("string");
                // Do something with the delivery data (The message payload)
    
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack send_webhook_event message");
            }
        };

        client.with_consumer(QUEUE, delegate).await;
        client.send_message(EXCHANGE, ROUTING_KEY, message).await;

    }
}

// Client::builder::new(URI)
//      .with_consumer("test1", |delegate1|)
//      .with_consumer("test2", |delegate2|)
//      .with_publisher("test2", |delegate2|)
