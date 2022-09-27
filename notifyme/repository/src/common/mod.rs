use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub repository_database_url: String,
    pub amqp_address: String,
    pub exchange: String,
    pub client_repository_request_queue: String,
    pub customer_repository_request_queue: String,
    pub repository_request_queue: String,
    pub client_repository_response_queue: String,
    pub customer_repository_response_queue: String,
    pub repository_response_queue: String,
}
