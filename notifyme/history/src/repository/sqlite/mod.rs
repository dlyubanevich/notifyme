use domain::models::{UserEventRecord, CustomerEventRecord};
use sqlx::SqlitePool;
use crate::repository::common::errors::DatabaseErrors;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub async fn new(url: &str) -> Result<Self, DatabaseErrors> {
        match SqlitePool::connect(url).await {
            Ok(pool) => Ok(SqliteRepository { pool }),
            Err(error) => Err(DatabaseErrors::ConnectionError(error.to_string())),
        }
    }
    pub async fn add_user_event(&mut self, record: UserEventRecord) -> Result<(),DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        // let result = sqlx::query!(
        //     r#"
        //     INSERT INTO users ( timestamp, user_id, data, comment )
        //     VALUES ( ?1, ?2, ?3, ?4 )
        //     "#,
        //     record.timestamp,
        //     record.user_id,
        //     record.data,
        //     record.comment
        // )
        // .execute(&mut connection)
        // .await;

        // match result {
        //     Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
        //     Ok(query_result) => Ok(()),
        // }

        Ok(())
    }

    pub async fn add_customer_event(&mut self, record: CustomerEventRecord) -> Result<(),DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        // let result = sqlx::query!(
        //     r#"
        //     INSERT INTO rooms ( name )
        //     VALUES ( ?1 )
        //     "#,
        //     room_name
        // )
        // .execute(&mut connection)
        // .await;

        // match result {
        //     Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
        //     Ok(query_result) => Ok(()),
        // }

        Ok(())
    }
}
