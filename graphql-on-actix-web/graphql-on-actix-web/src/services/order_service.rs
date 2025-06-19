use crate::models::order_model::{NewOrder, Order, OrderDB, UpdateOrder};
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

    pub async fn update_order(pool: &PgPool, request: UpdateOrder) -> Result<Option<Order>, sqlx::Error> {
        let result = sqlx::query_as::<_, OrderDB>(
            r#"
            UPDATE orders 
            SET order_name = COALESCE($2, order_name), 
                user_id = COALESCE($3, user_id),
                order_price = COALESCE($4, order_price),
                order_status = COALESCE($5, order_status)
            WHERE order_id = $1
            RETURNING *
            "#
        )
        .bind(request.order_id)
        .bind(request.order_name)
        .bind(request.user_id)
        .bind(request.order_price)
        .bind(request.order_status)
        .fetch_optional(pool)
        .await?;
        
        Ok(result.map(Order::from))
    }

    pub async fn delete_order(pool: &PgPool, order_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM orders WHERE order_id = $1")
            .bind(order_id)
            .execute(pool)
            .await?;

        // Mengecek berapa row yang kena delete
        Ok(result.rows_affected() > 0)
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

