use std::sync::Arc;

use lapin::{Channel, Connection, ConnectionProperties};
use tokio::sync::Mutex;

use crate::core::{Consumer, IncomingMessageHandler, Publisher};
use crate::error::RabbitMqClientError;

pub struct RabbitMqClient {
    connection: Connection,
    publisher: Option<Arc<Publisher>>,
    consumers: Vec<Consumer>,
}
impl<'a> RabbitMqClient {
    pub fn builder() -> RabbitMqClientBuilder<'a> {
        RabbitMqClientBuilder::new()
    }
    pub fn get_publisher(&self) -> Result<Arc<Publisher>, RabbitMqClientError> {
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

pub struct RabbitMqClientBuilder<'a> {
    options: ConnectionProperties,
    publisher: Option<BuildPublisher>,
    consumers: Vec<BuildConsumer<'a>>,
}
impl<'a> RabbitMqClientBuilder<'a> {
    pub fn new() -> Self {
        RabbitMqClientBuilder::default()
    }
    pub fn with_publisher(mut self) -> Self {
        self.publisher = Some(BuildPublisher {});
        self
    }
    pub fn with_consumer<F: IncomingMessageHandler + 'static>(
        mut self,
        queue: &'a str,
        message_handler: F,
    ) -> Self {
        let consumer = BuildConsumer {
            queue,
            message_handler: Arc::new(Mutex::new(message_handler)),
        };
        self.consumers.push(consumer);
        self
    }

    pub async fn build(self, uri: &str) -> Result<RabbitMqClient, RabbitMqClientError> {
        let mut connection = match Connection::connect(uri, self.options).await {
            Ok(connection) => connection,
            Err(error) => return Err(RabbitMqClientError::BuildConnectionError(error.to_string())),
        };
        let publisher = if self.publisher.is_some() {
            let channel = Self::create_channel(&mut connection).await?;
            Some(Arc::new(Publisher::new(channel)))
        } else {
            None
        };
        let mut consumers = Vec::with_capacity(self.consumers.len());
        for consumer in self.consumers.iter() {
            let channel = Self::create_channel(&mut connection).await?;
            consumers.push(
                Consumer::new2(channel, consumer.queue, consumer.message_handler.clone()).await,
            );
        }

        Ok(RabbitMqClient {
            connection,
            publisher,
            consumers,
        })
    }
    async fn create_channel(connection: &mut Connection) -> Result<Channel, RabbitMqClientError> {
        match connection.create_channel().await {
            Ok(channel) => Ok(channel),
            Err(error) => Err(RabbitMqClientError::BuildChannelFailure(error.to_string())),
        }
    }
}

impl<'a> Default for RabbitMqClientBuilder<'a> {
    fn default() -> Self {
        let options = ConnectionProperties::default()
            // Use tokio executor and reactor.
            // At the moment the reactor is only available for unix.
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);
        RabbitMqClientBuilder {
            publisher: None,
            consumers: vec![],
            options,
        }
    }
}

struct BuildPublisher;

struct BuildConsumer<'a> {
    queue: &'a str,
    message_handler: Arc<Mutex<dyn IncomingMessageHandler>>,
}
