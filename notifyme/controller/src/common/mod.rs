use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub amqp_address: String,
    pub exchange: String,
    pub history_queue: String,
    pub client_request_queue: String,
    pub customer_request_queue: String,
    pub client_repository_request_queue: String,
    pub customer_repository_request_queue: String,
    pub repository_request_queue: String,
    pub client_response_queue: String,
    pub customer_response_queue: String,
    pub client_repository_response_queue: String,
    pub customer_repository_response_queue: String,
    pub repository_response_queue: String,
}
