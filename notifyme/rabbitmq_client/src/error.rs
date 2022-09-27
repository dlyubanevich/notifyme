use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageBrokerError {
    #[error("Build connection error: [{0}]")]
    BuildConnectionError(String),
    #[error("Build channel error: [{0}]")]
    BuildChannelFailure(String),
    #[error("No publisher. You must build client with publisher, to get the publisher!")]
    PublisherFailure,
    #[error("Creating consumer error: [{0}]")]
    CreatingConsumerFailure(String),
    #[error("Publish message error: [{0}]")]
    PublishMessageFailure(String),
}
