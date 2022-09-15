use async_trait::async_trait;

use crate::repository::common::errors::DatabaseErrors;

#[async_trait]
pub trait Repository {
    async fn push(&mut self, record: String) -> Result<(), DatabaseErrors>;
}
