pub mod storage;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub amqp_address: String,
    pub exchange: String,
    pub client_request_queue: String,
    pub customer_request_queue: String,
    pub client_response_queue: String,
    pub customer_response_queue: String,
    pub telegram_client_token: String,
    pub telegram_client_url: String,
    pub telegram_client_address: String,
    pub telegram_customer_token: String,
    pub telegram_customer_url: String,
    pub telegram_customer_address: String,
}
