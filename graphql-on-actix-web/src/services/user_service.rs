use std::collections::HashMap;

use crate::models::{order_model::{OrderDB}, user_model::{NewUser, User, UserDB, UserWithOrderRow, UserWithOrders}};
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users_db = sqlx::query_as::<_, UserDB>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        
        Ok(users_db.into_iter().map(User::from).collect())
    }

    pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as::<_, UserDB>(
            "SELECT * FROM users WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool) // pakai optional karena bisa tidak ada data
        .await?;
        
        Ok(result.map(User::from)) // convert Option<UserDB> ke Option<User>
    }
    
    pub async fn create_user(pool: &PgPool, request: NewUser) -> Result<User, sqlx::Error> {
        let user_db = sqlx::query_as::<_, UserDB>(
            "INSERT INTO users (email, full_name) VALUES ($1, $2) RETURNING *"
        )
        .bind(request.email)
        .bind(request.full_name)
        .fetch_one(pool)
        .await?;
        
        Ok(User::from(user_db))
    }

    pub async fn get_users_with_orders(pool: &PgPool) -> Result<Vec<UserWithOrders>, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithOrderRow>(
            r#"
            SELECT 
                u.user_id AS user_id,
                u.email AS user_email,
                u.full_name AS user_full_name,
                o.order_id AS order_id,
                o.order_name AS order_name,
                o.order_price AS order_price,
                o.order_date AS order_date,
                o.order_status AS order_status,
                o.last_update AS last_update
            FROM users u
            LEFT JOIN orders o ON u.user_id = o.user_id
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut map: HashMap<i32, UserWithOrders> = HashMap::new();

        for row in rows {
            // entry per user
            let user_entry = map.entry(row.user_id).or_insert_with(|| UserWithOrders {
                orders: Vec::new(),
                user_id: row.user_id,
                email: row.user_email.clone(),
                full_name: row.user_full_name.clone(),
            });

            // kalau order ada
            if let Some(order_id) = row.order_id {
                user_entry.orders.push(OrderDB {
                    order_id,
                    order_name: row.order_name.clone().unwrap(),
                    order_price: row.order_price.unwrap(),
                    order_date: row.order_date,
                    order_status: row.order_status.clone().unwrap(),
                    last_update: row.last_update,
                    user_id: row.user_id,
                }.into());
            }
        }

        Ok(map.into_values().collect())
    }

}
