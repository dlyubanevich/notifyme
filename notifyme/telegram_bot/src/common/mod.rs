pub mod storage;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;