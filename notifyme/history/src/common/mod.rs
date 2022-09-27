use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub amqp_address: String,
    pub history_database_url: String,
    pub history_queue: String,
}
