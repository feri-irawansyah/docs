use crate::models::{order_model::{NewOrder, Order, OrderDB}};
use sqlx::PgPool;
use async_graphql::dataloader::Loader;
use std::{collections::HashMap, sync::Arc};

pub struct OrderService;

impl OrderService {
    pub async fn create_order(pool: &PgPool, request: NewOrder) -> Result<Order, sqlx::Error> {
        let order_db = sqlx::query_as::<_, OrderDB>(r#"INSERT INTO 
            orders (order_name, user_id, order_date, order_price, order_status, last_update) 
            VALUES ($1, $2, $3, $4, 'pending', $5) RETURNING *"#)
            .bind(request.order_name)
            .bind(request.user_id)
            .bind(chrono::Utc::now().naive_utc())
            .bind(request.order_price)
            .bind(chrono::Utc::now().naive_utc())
            .fetch_one(pool)
            .await?;
        Ok(Order::from(order_db))
    }

    pub async fn get_orders(pool: &PgPool, user_id: i32) -> Result<Vec<Order>, sqlx::Error> {
        let orders_db = sqlx::query_as::<_, OrderDB>("SELECT * FROM orders WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(orders_db.into_iter().map(Order::from).collect())
    }

}

pub struct OrderLoader {
    pub pool: sqlx::PgPool,
}

impl Loader<i32> for OrderLoader {
    type Value = Vec<Order>; // ⬅️ Harus Vec<Order> BUKAN String
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        // Ambil semua order yang user_id ada di keys
        let pool = &self.pool;
        let orders_db = sqlx::query_as::<_, OrderDB>(
            "SELECT * FROM orders WHERE user_id = ANY($1)"
        )
        .bind(keys)
        .fetch_all(pool)
        .await?;

        // Build map: user_id => Vec<Order>
        let mut order_map: HashMap<i32, Vec<Order>> = HashMap::new();
        for order in orders_db {
            order_map
                .entry(order.user_id)
                .or_insert_with(Vec::new)
                .push(Order::from(order));
        }

        Ok(order_map)
    }
}

