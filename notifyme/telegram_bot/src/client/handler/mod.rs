use std::sync::Arc;

use domain::responses::ClientResponse;
use rabbitmq_client::IncomingMessageHandler;
use tokio::sync::Mutex;

use crate::client::ClientService;
pub struct MessageHandler {}
impl MessageHandler {
    pub fn client_response(
        service: Arc<Mutex<ClientService>>,
    ) -> impl IncomingMessageHandler + 'static {
        move |message: String| {
            let service = service.clone();
            async move {
                let response: ClientResponse = serde_json::from_str(&message).unwrap();
                if let Err(error) = service.lock().await.handle_response(response).await {
                    log::error!("Message handler client response error: {}", error);
                };
            }
        }
    }
}
