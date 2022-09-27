use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties};
use tokio_stream::StreamExt;

use crate::error::MessageBrokerError;
use crate::{IncomingMessageHandler, Publisher};

pub struct RabbitMqManager {
    connection: Connection,
}
impl RabbitMqManager {
    pub fn builder() -> RabbitMqClientBuilder {
        RabbitMqClientBuilder::new()
    }
    pub async fn add_consumer<F: IncomingMessageHandler + 'static>(
        &mut self,
        queue: &str,
        message_handler: F,
    ) -> Result<(), MessageBrokerError> {
        let channel = create_channel(&self.connection).await?;
        let mut consumer = match channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(consumer) => consumer,
            Err(error) => {
                return Err(MessageBrokerError::CreatingConsumerFailure(
                    error.to_string(),
                ))
            }
        };
        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                let delivery = match delivery {
                    Ok(delivery) => delivery,
                    Err(error) => {
                        log::info!("Failed to consume queue message {}", error);
                        continue;
                    }
                };
                let message = String::from_utf8_lossy(&delivery.data).to_string();
                log::info!("Received message from queue [{}]:[{}]", consumer.queue(), message);
                message_handler.handle_message(message).await;
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack message");
            }
        });
        Ok(())
    }
    pub async fn get_publisher(&self) -> Result<Publisher, MessageBrokerError> {
        let channel = create_channel(&self.connection).await?;
        Ok(Publisher::new(channel))
    }
    pub fn run(self) {
        self.connection.run().expect("connection run");
    }
}

pub struct RabbitMqClientBuilder {
    options: ConnectionProperties,
}
impl RabbitMqClientBuilder {
    pub fn new() -> Self {
        RabbitMqClientBuilder::default()
    }
    pub async fn build(self, uri: &str) -> Result<RabbitMqManager, MessageBrokerError> {
        match Connection::connect(uri, self.options).await {
            Ok(connection) => Ok(RabbitMqManager { connection }),
            Err(error) => Err(MessageBrokerError::BuildConnectionError(error.to_string())),
        }
    }
}

async fn create_channel(connection: &Connection) -> Result<Channel, MessageBrokerError> {
    match connection.create_channel().await {
        Ok(channel) => Ok(channel),
        Err(error) => Err(MessageBrokerError::BuildChannelFailure(error.to_string())),
    }
}

impl Default for RabbitMqClientBuilder {
    fn default() -> Self {
        let options = ConnectionProperties::default()
            // Use tokio executor and reactor.
            // At the moment the reactor is only available for unix.
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);
        RabbitMqClientBuilder { options }
    }
}
