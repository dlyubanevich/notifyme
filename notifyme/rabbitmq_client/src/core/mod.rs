use std::sync::Arc;

use lapin::{Connection, ConnectionProperties, ConsumerDelegate};
use tokio::sync::Mutex;

mod client;
pub mod consumer;
mod handler;
pub mod publisher;

pub use client::RabbitMqClient;
pub use consumer::Consumer;
pub use handler::IncomingMessageHandler;
pub use publisher::Publisher;

pub struct Client {
    connection: Connection,
    consumers: Vec<consumer::Consumer>,
    publisher: Arc<Mutex<publisher::Publisher>>,
}

impl Client {
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
        self.publisher
            .lock()
            .await
            .publish_message(exchange, rooting_key, message)
            .await;
    }

    pub async fn with_consumer(&mut self, queue: &str, delegate: impl ConsumerDelegate + 'static) {
        let channel = self.connection.create_channel().await.unwrap();
        let consumer = consumer::Consumer::new(channel, queue, delegate).await;
        self.consumers.push(consumer);
    }

    pub fn run(self) {
        self.connection.run().expect("connection run");
    }
}

#[cfg(test)]
mod tests {}
