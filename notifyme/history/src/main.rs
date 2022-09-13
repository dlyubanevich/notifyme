use std::sync::Arc;

use history::{response_delegate, request_delegate, error_delegate, MessageHandler};
use message_queue::Client;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {

    let message_handler = Arc::new(Mutex::new(MessageHandler::new()));
    
    let mut client = Client::new("amqp://localhost:5672").await;
    client.with_consumer("request", request_delegate(message_handler.clone())).await;
    client.with_consumer("response", response_delegate(message_handler.clone())).await;
    client.with_consumer("error", error_delegate(message_handler.clone())).await;

    client.run();

    //TODO
    // Раскидать все по папкам
    // Добавить SqliteRepository с записью всех сообщений
    // Сделать через async трейт Repository, для того чтобы можно было использовать Mongo/Sqlite в зависимости от фич
}