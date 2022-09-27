use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseErrors {
    #[error("Connection failure: [{0}]")]
    ConnectionFailure(String),
    #[error("Request error: [{0}]")]
    RequestError(String),
    #[error("Transaction error: [{0}]")]
    TransactionError(String),
}
