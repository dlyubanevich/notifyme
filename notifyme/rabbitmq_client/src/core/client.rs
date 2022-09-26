use std::sync::Arc;

use lapin::{Channel, Connection, ConnectionProperties};
use tokio::sync::Mutex;

use crate::core::{Consumer, IncomingMessageHandler, Publisher};
use crate::error::RabbitMqClientError;

pub struct RabbitMqClient {
    connection: Connection,
    publisher: Option<Arc<Mutex<Publisher>>>,
    consumers: Vec<Consumer>,
}
impl RabbitMqClient {
    pub fn builder() -> RabbitMqClientBuilder {
        RabbitMqClientBuilder::new()
    }
    pub async fn add_consumer<F: IncomingMessageHandler + 'static>(&mut self, queue: & str, message_handler: F) {
        let channel = create_channel(&mut self.connection).await.unwrap();
        let consumer = Consumer::new2(channel, queue, Arc::new(Mutex::new(message_handler))).await;
        self.consumers.push(consumer);
    }
    pub fn get_publisher(&self) -> Result<Arc<Mutex<Publisher>>, RabbitMqClientError> {
        match &self.publisher {
            Some(publisher) => Ok(Arc::clone(publisher)),
            None => Err(RabbitMqClientError::PublisherFailure),
        }
    }
    pub fn close(&mut self) {
        self.consumers.clear();
        self.publisher = None;
    }
    pub fn run(self) {
        self.connection.run().expect("connection run");
    }
}

pub struct RabbitMqClientBuilder {
    options: ConnectionProperties,
    publisher: Option<BuildPublisher>,
}
impl RabbitMqClientBuilder {
    pub fn new() -> Self {
        RabbitMqClientBuilder::default()
    }
    pub fn with_publisher(mut self) -> Self {
        self.publisher = Some(BuildPublisher {});
        self
    }

    pub async fn build(self, uri: &str) -> Result<RabbitMqClient, RabbitMqClientError> {
        let mut connection = match Connection::connect(uri, self.options).await {
            Ok(connection) => connection,
            Err(error) => return Err(RabbitMqClientError::BuildConnectionError(error.to_string())),
        };
        let publisher = if self.publisher.is_some() {
            let channel = create_channel(&mut connection).await?;
            Some(Arc::new(Mutex::new(Publisher::new(channel))))
        } else {
            None
        };
        let consumers = Vec::new();

        Ok(RabbitMqClient {
            connection,
            publisher,
            consumers,
        })
    }

}

async fn create_channel(connection: &mut Connection) -> Result<Channel, RabbitMqClientError> {
    match connection.create_channel().await {
        Ok(channel) => Ok(channel),
        Err(error) => Err(RabbitMqClientError::BuildChannelFailure(error.to_string())),
    }
}

impl Default for RabbitMqClientBuilder {
    fn default() -> Self {
        let options = ConnectionProperties::default()
            // Use tokio executor and reactor.
            // At the moment the reactor is only available for unix.
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);
        RabbitMqClientBuilder {
            publisher: None,
            options,
        }
    }
}

struct BuildPublisher;
