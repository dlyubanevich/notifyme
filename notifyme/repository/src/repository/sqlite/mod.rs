use std::collections::HashMap;

use domain::models::{Notification, Customer, Product};
use sqlx::SqlitePool;

use crate::repository::common::errors::DatabaseErrors;

pub struct SqliteRepository {
    pool: SqlitePool,
    hash_data: HashData,
}

impl SqliteRepository {
    pub async fn new(url: &str) -> Result<Self, DatabaseErrors> {
        match SqlitePool::connect(url).await {
            Ok(pool) => {
                let hash_data = HashData::new(&pool).await;
                Ok(SqliteRepository { pool, hash_data })
            },
            Err(error) => Err(DatabaseErrors::ConnectionProblem(error.to_string())),
        }
    }

    pub async fn get_customers(&self) -> Result<Vec<Customer>, DatabaseErrors> {
        Ok(self.hash_data.get_customers())
    }

    pub async fn get_products(&self, customer: &str) -> Result<Vec<Product>, DatabaseErrors> {
        Ok(self.hash_data.get_customer_products(customer))
    }

    pub async fn add_subscription(
        &mut self,
        user_id: u32,
        customer: &str,
        product: &str,
    ) -> Result<(), DatabaseErrors> {
        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };

        let customer_id = self.hash_data.get_customer_id(customer);
        let product_id = self.hash_data.get_product_id(customer, product);
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
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        let subscription_id = result.unwrap().last_insert_rowid();

        let result = sqlx::query!(
            r#"
            INSERT INTO 
                active_subscriptions ( subscription_id )
            VALUES 
                ( ?1 )
            "#,
            subscription_id
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
        &mut self,
        user_id: u32,
        key: String,
    ) -> Result<Option<Customer>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT 
                name 
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

        let mut customers: Vec<String> = result
            .unwrap()
            .iter()
            .map(|record| record.name.to_string())
            .collect();

            let customer = match customers.pop() {
                Some(customer) => customer,
                None => return Ok(None),
            };
    
            let mut transaction = match self.pool.begin().await {
                Ok(transaction) => transaction,
                Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
            };
    
            let customer_id = self.hash_data.get_customer_id(&customer);
            let result = sqlx::query!(
                r#"
                INSERT INTO 
                    users_customers ( user_id, customer_id )
                VALUES 
                    ( ?1, ?2 )
                "#,
                user_id,
                customer_id
            )
            .execute(&mut transaction)
            .await;
    
            if let Err(error) = result {
                return Err(DatabaseErrors::RequestError(error.to_string()));
            }
            if let Err(error) = transaction.commit().await {
                return Err(DatabaseErrors::TransactionError(error.to_string()));
            }
    
            self.hash_data.insert_user_for_customer(&customer, user_id);
            Ok(Some(Customer { name: customer }))
        
    }

    pub async fn get_products_for_notification(
        &self,
        customer: &str,
    ) -> Result<Vec<Product>, DatabaseErrors> {
        let customer_id = self.hash_data.get_customer_id(customer);
        let result = sqlx::query!(
            r#"
            SELECT DISTINCT
                products.name
            FROM 
                subscriptions
                    INNER JOIN active_subscriptions 
                    ON subscriptions.id = active_subscriptions.subscription_id 
                    LEFT JOIN products
                    ON subscriptions.product_id = products.id
            WHERE
                subscriptions.customer_id = ?1               
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
            .map(|record| Product { name: record.name.to_string() } )
            .collect())
    }
    
    pub async fn add_notification(
        &mut self,
        customer: &str,
        product: &str,
        text: String,
    ) -> Result<(), DatabaseErrors> {
        let mut transaction = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
        };
        
        let customer_id = self.hash_data.get_customer_id(customer);
        let product_id = self.hash_data.get_product_id(customer, product);
        let result = sqlx::query!(
            r#"
            INSERT INTO 
                notifications ( customer_id, product_id, text )
            VALUES 
                ( ?1, ?2, ?3 )
            "#,
            customer_id,
            product_id,
            text
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
    pub async fn get_notifications(&mut self, customer: &str, product: &str, notification: String) -> Result<Vec<Notification>, DatabaseErrors> {
        let customer_id = self.hash_data.get_customer_id(customer);
        let product_id = self.hash_data.get_product_id(customer, product);
        let result = sqlx::query!(
            r#"
            SELECT
                subscriptions.id,
                subscriptions.user_id as user_id,
                customers.name as customer,
                products.name as product
            FROM 
                subscriptions
                    INNER JOIN active_subscriptions 
                    ON subscriptions.id = active_subscriptions.subscription_id 
                    INNER JOIN customers
                    ON customers.id = subscriptions.customer_id
                        AND customers.id = ?1
                    INNER JOIN products
                    ON products.id = subscriptions.product_id
                        AND products.id = ?2           
            "#,
            customer_id,
            product_id
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        let result = result.unwrap();
        let notifications = result
            .iter()
            .map(|record| Notification { user_id: record.user_id as u32, customer: record.customer.to_string(), product: record.product.to_string(), text: notification.clone()} )
            .collect();

        let subscriptions_to_delete: Vec<String> = result
            .iter()
            .map(|record| record.id.to_string())
            .collect();
        let subscriptions_to_delete = subscriptions_to_delete.join(",");

            let mut transaction = match self.pool.begin().await {
                Ok(transaction) => transaction,
                Err(error) => return Err(DatabaseErrors::TransactionError(error.to_string())),
            };

            let result = sqlx::query!(
                r#"
                DELETE FROM 
                    active_subscriptions
                WHERE 
                    active_subscriptions.subscription_id IN ( ?1 )
                "#,
                subscriptions_to_delete
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

    pub async fn get_customers_user_id(&mut self, customer: &str) -> Result<u32, DatabaseErrors> {
        Ok(self.hash_data.get_user_for_customer(customer).unwrap())
    }
}

struct HashData {
    customers: HashMap<String, u32>,
    products: HashMap<String, HashMap<String, u32>>,
    users_customers: HashMap<String, u32>,
}
impl HashData {
    async fn new(pool: &SqlitePool) -> Self {
        let customers = Self::get_all_customers(pool).await;
        let products = Self::get_all_products(pool).await;
        let users_customers = Self::get_users_customers(pool).await;
        HashData { customers, products, users_customers }    
    }
    fn get_customer_id(&self, customer: &str) -> Option<u32>  {
        self.customers.get(customer).cloned()
    }
    fn get_product_id(&self, customer: &str, product: &str) -> Option<u32> {
        self.products.get(customer).unwrap().get(product).cloned()
    }
    fn get_user_for_customer(&self, customer: &str) -> Option<u32> {
        self.users_customers.get(customer).cloned()
    }
    fn insert_user_for_customer(&mut self, customer: &str, user_id: u32) {
        self.users_customers.insert(customer.to_string(), user_id);
    }
    fn get_customers(&self) -> Vec<Customer> {
        self.customers.keys().into_iter().map(|key| Customer { name: key.to_string() } ).collect()
    }
    fn get_customer_products(&self, customer: &str) -> Vec<Product> {
        self.products.get(customer).unwrap().keys().into_iter().map(|key| Product { name: key.to_string() } ).collect()
    }
    async fn get_all_customers(pool: &SqlitePool) -> HashMap<String, u32>{
        let result = sqlx::query!(
            r#"
            SELECT 
                id, name 
            FROM 
                customers 
            "#
        )
        .fetch_all(pool)
        .await;

        result
            .unwrap()
            .iter()
            .map(|record| (record.name.to_string(), record.id as u32))
            .collect()
    }

    async fn get_all_products(pool: &SqlitePool) -> HashMap<String, HashMap<String, u32>> {
        let result = sqlx::query!(
            r#"
            SELECT 
                customers.name as customer,
                products.id,
                products.name 
            FROM 
                products 
                    INNER JOIN customers_products 
                    ON products.id = customers_products.product_id
                    INNER JOIN customers
                    ON customers_products.customer_id = customers.id
            "#
        )
        .fetch_all(pool)
        .await;

        let mut products = HashMap::<String, HashMap<String, u32>>::new();
        for rec in result.unwrap() {
            let customer_products = match products.get_mut(&rec.customer) {
                Some(customer_products) => customer_products,
                None => {
                    let customer_products = HashMap::<String, u32>::new();  
                    products.insert(rec.customer.clone(), customer_products);
                    products.get_mut(&rec.customer).unwrap()
                },
            };
            customer_products.insert(rec.name, rec.id as u32);
        }

        products
    }

    async fn get_users_customers(pool: &SqlitePool) -> HashMap<String, u32>{
        let result = sqlx::query!(
            r#"
            SELECT 
                customers.name,
                users_customers.user_id 
            FROM 
                users_customers
                LEFT JOIN customers
                ON users_customers.customer_id = customers.id
            "#
        )
        .fetch_all(pool)
        .await;

        result
            .unwrap()
            .iter()
            .map(|record| (record.name.to_string(), record.user_id as u32))
            .collect()
    }
}