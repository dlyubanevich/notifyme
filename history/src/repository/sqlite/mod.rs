use crate::repository::error::DatabaseErrors;
use domain::models::{CustomerEventRecord, UserEventRecord};
use sqlx::SqlitePool;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub async fn new(url: &str) -> Result<Self, DatabaseErrors> {
        match SqlitePool::connect(url).await {
            Ok(pool) => Ok(SqliteRepository { pool }),
            Err(error) => Err(DatabaseErrors::ConnectionFailure(error.to_string())),
        }
    }
    pub async fn add_user_event(&mut self, record: UserEventRecord) -> Result<(), DatabaseErrors> {
        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO users ( timestamp, user_id, data, event )
            VALUES ( ?1, ?2, ?3, ?4 )
            "#,
            record.timestamp,
            record.user_id,
            record.data,
            record.event
        )
        .execute(&mut transaction)
        .await;

        if let Err(error) = transaction.commit().await {
            return Err(DatabaseErrors::TransactionError(error.to_string()));
        }

        match result {
            Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
            Ok(_) => Ok(()),
        }
    }

    pub async fn add_customer_event(
        &mut self,
        record: CustomerEventRecord,
    ) -> Result<(), DatabaseErrors> {
        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };
        let result = sqlx::query!(
            r#"
            INSERT INTO customers ( timestamp, user_id, customer, data, event )
            VALUES ( ?1, ?2, ?3, ?4, ?5 )
            "#,
            record.timestamp,
            record.user_id,
            record.customer,
            record.data,
            record.event
        )
        .execute(&mut transaction)
        .await;
        if let Err(error) = transaction.commit().await {
            return Err(DatabaseErrors::TransactionError(error.to_string()));
        }
        match result {
            Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
            Ok(_) => Ok(()),
        }
    }
}
