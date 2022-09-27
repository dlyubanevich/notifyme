use lapin::{options::BasicPublishOptions, BasicProperties, Channel};

use crate::error::MessageBrokerError;

pub struct Publisher {
    channel: Channel,
}
impl Publisher {
    pub fn new(channel: Channel) -> Self {
        Publisher { channel }
    }
    pub async fn publish_message(
        &self,
        exchange: &str,
        rooting_key: &str,
        message: String,
    ) -> Result<(), MessageBrokerError> {
        let publisher_confirm = match self
            .channel
            .basic_publish(
                exchange,
                rooting_key,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
        {
            Ok(confirm) => confirm,
            Err(error) => return Err(MessageBrokerError::PublishMessageFailure(error.to_string())),
        };

        match publisher_confirm.await {
            Ok(_) => Ok(()),
            Err(error) => Err(MessageBrokerError::PublishMessageFailure(error.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {}
