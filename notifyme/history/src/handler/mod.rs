use std::sync::Arc;

use domain::records::Record;
use rabbitmq_client::IncomingMessageHandler;
use tokio::sync::Mutex;

use crate::HistoryService;

pub struct MessageHandler {}
impl MessageHandler {
    pub fn record(service: Arc<Mutex<HistoryService>>) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let record: Record = serde_json::from_str(&message).unwrap();
                service.lock().await.add_record(record).await;
            }
        }
    }
}
