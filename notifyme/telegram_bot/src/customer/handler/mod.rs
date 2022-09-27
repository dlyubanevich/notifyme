use std::sync::Arc;

use domain::responses::CustomerResponse;
use rabbitmq_client::IncomingMessageHandler;
use tokio::sync::Mutex;

use crate::customer::CustomerService;

pub struct MessageHandler {}
impl MessageHandler {
    pub fn customer_response(
        service: Arc<Mutex<CustomerService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let response: CustomerResponse = serde_json::from_str(&message).unwrap();
                if let Err(error) = service.lock().await.handle_response(response).await {
                    log::error!("Message handler customer response error: {}", error);
                }
            }
        }
    }
}
