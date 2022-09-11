use lapin::{Connection, ConnectionProperties, Channel, options::BasicPublishOptions, BasicProperties};

mod consumer;
mod model;
mod publisher;

pub struct Client {
    connection: Connection,
    consumer: consumer::Consumer,
    publisher: publisher::Publisher
}

impl Client {
    pub async fn new(uri: &str) -> Self {
        let options = ConnectionProperties::default()
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

        let connection = Connection::connect(uri, options).await.unwrap();
        let consumer_channel = connection.create_channel().await.unwrap();
        let publisher_channel = connection.create_channel().await.unwrap();

        let consumer = consumer::Consumer::new(consumer_channel);
        let publisher = publisher::Publisher::new(publisher_channel);

        Client {
            connection, 
            consumer,
            publisher    
        }
    }
    pub async fn send_message(&mut self, exchange: &str, rooting_key: &str, message: String) {
        self.publisher.publish_message(exchange, rooting_key, message).await;  
        //TODO Тут вся проверка отправки сообщения. Если возвращаемое значение не Ок, то нужно хранить эти сообщения где-нибудь для последующей отправки 
    }

    //TODO 
    // Consumer - передавать канал по ссылке и создавать консумера из Lapin.
    // Добавить к консумеру установку делегата на чтение очереди
    // Все это обернуть в билдера
    // Развернуть RabbitMQ в Докере и написать тесты, на отправку и получение сообщений

}
