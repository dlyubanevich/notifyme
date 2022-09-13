use lapin::{options::BasicPublishOptions, BasicProperties, Channel};

pub struct Publisher {
    channel: Channel,
}
impl Publisher {
    pub fn new(channel: Channel) -> Self {
        Publisher { channel }
    }
    pub async fn publish_message(&mut self, exchange: &str, rooting_key: &str, message: String) {
        self.channel
            .basic_publish(
                exchange,
                rooting_key,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {}
