use domain::models::{Customer, Notification, Product};
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
    pub async fn get_customers(&self) -> Result<Vec<Customer>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                id, name 
            FROM 
                customers 
            "#
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        Ok(result
            .unwrap()
            .iter()
            .map(|record| Customer {
                id: record.id as u32,
                name: record.name.to_string(),
            })
            .collect())
    }
    pub async fn get_products(&self, customer_id: u32) -> Result<Vec<Product>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                id, name 
            FROM 
                products 
                INNER JOIN customers_products 
                ON products.id = customers_products.product_id
                    AND customers_products.customer_id = ?1
            "#,
            customer_id
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        Ok(result
            .unwrap()
            .iter()
            .map(|record| Product {
                id: record.id as u32,
                name: record.name.to_string(),
            })
            .collect())
    }
    pub async fn add_subscription(
        &mut self,
        user_id: u32,
        customer_id: u32,
        product_id: u32,
    ) -> Result<(), DatabaseErrors> {
        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO 
                subscriptions ( user_id, customer_id, product_id )
            VALUES 
                ( ?1, ?2, ?3 )
            "#,
            user_id,
            customer_id,
            product_id
        )
        .execute(&mut transaction)
        .await;

        if let Err(error) = result {
            if let Err(error) = transaction.rollback().await {
                return Err(DatabaseErrors::TransactionError(error.to_string()));
            }
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        if let Err(error) = transaction.commit().await {
            return Err(DatabaseErrors::TransactionError(error.to_string()));
        }

        Ok(())
    }
    pub async fn try_authorize(
        &self,
        user_id: u32,
        key: String,
    ) -> Result<Option<Customer>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                id, name 
            FROM 
                customers 
            WHERE
                key = ?1   
            "#,
            key
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        let mut customers: Vec<Customer> = result
            .unwrap()
            .iter()
            .map(|record| Customer {
                id: record.id as u32,
                name: record.name.to_string(),
            })
            .collect();

        let customer = match customers.pop() {
            Some(customer) => customer,
            None => return Ok(None),
        };

        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO 
                users_customers ( user_id, customer_id )
            VALUES 
                ( ?1, ?2 )
            "#,
            user_id,
            customer.id
        )
        .execute(&mut transaction)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        if let Err(error) = transaction.commit().await {
            return Err(DatabaseErrors::TransactionError(error.to_string()));
        }

        Ok(Some(customer))
    }
    pub async fn get_subscriptions(
        &self,
        customer_id: u32,
    ) -> Result<Vec<Product>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                products.id,
                products.name
            FROM 
                products
                    INNER JOIN subscriptions 
                    ON subscriptions.product_id = products.id
                        AND subscriptions.customer_id = ?1
            "#,
            customer_id
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        Ok(result
            .unwrap()
            .iter()
            .map(|record| Product {
                id: record.id as u32,
                name: record.name.to_string(),
            })
            .collect())
    }
    pub async fn add_notification(
        &mut self,
        customer_id: u32,
        product_id: u32,
        text: String,
    ) -> Result<Vec<Notification>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                subscriptions.user_id,
                products.id AS product_id,
                products.name AS product_name,
                customers.id AS customer_id,
                customers.name AS customer_name
            FROM 
                subscriptions 
                    INNER JOIN products ON subscriptions.product_id = products.id
                    INNER JOIN customers ON subscriptions.customer_id = customers.id
            WHERE
                customer_id = ?1 AND product_id = ?2   
            "#,
            customer_id,
            product_id
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        let notifications: Vec<Notification> = result
            .unwrap()
            .iter()
            .map(|record| Notification {
                user_id: record.user_id as u32,
                customer: Customer {
                    id: record.customer_id as u32,
                    name: record.customer_name.to_string(),
                },
                product: Product {
                    id: record.product_id as u32,
                    name: record.product_name.to_string(),
                },
                text: text.clone(),
            })
            .collect();

        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            DELETE 
            FROM 
                subscriptions 
            WHERE 
                customer_id = ?1 AND product_id = ?2 
            "#,
            customer_id,
            product_id
        )
        .execute(&mut transaction)
        .await;

        if let Err(error) = result {
            if let Err(error) = transaction.rollback().await {
                return Err(DatabaseErrors::TransactionError(error.to_string()));
            }
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        if let Err(error) = transaction.commit().await {
            return Err(DatabaseErrors::TransactionError(error.to_string()));
        }

        Ok(notifications)
    }
}
