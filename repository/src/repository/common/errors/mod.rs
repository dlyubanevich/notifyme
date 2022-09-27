use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseErrors {
    #[error("Connection error: [{0}]")]
    ConnectionProblem(String),
    #[error("Transaction error: [{0}]")]
    TransactionError(String),
    #[error("Request error: [{0}]")]
    RequestError(String),
}
