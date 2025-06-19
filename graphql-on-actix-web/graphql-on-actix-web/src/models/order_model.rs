use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::graphql::datetime::MyDateTime;

#[derive(Debug, Deserialize, InputObject)]
pub struct NewOrder {
    pub order_name: String,
    pub user_id: i32,
    pub order_price: f64,
}

#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct UpdateOrder {
    pub order_id: i32,
    pub user_id: Option<i32>,
    pub order_name: Option<String>,
    pub order_price: Option<f64>,
    pub order_status: Option<String>,
}

#[derive(FromRow, Deserialize, Debug, Serialize)]
pub struct OrderDB {
    pub order_id: i32,
    pub order_name: String,
    pub user_id: i32,
    pub order_date: Option<chrono::DateTime<chrono::Utc>>,
    pub order_price: f64,
    pub order_status: String,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(SimpleObject, Clone)]
pub struct Order {
    pub order_id: i32,
    pub order_name: String,
    pub user_id: i32,
    pub order_date: Option<MyDateTime>,
    pub order_price: f64,
    pub order_status: String,
    pub last_update: Option<MyDateTime>,
}

impl From<OrderDB> for Order {
    fn from(user: OrderDB) -> Self {
        Order {
            order_id: user.order_id,
            order_name: user.order_name,
            user_id: user.user_id,
            order_date: Some(MyDateTime(user.order_date.unwrap_or_default())),
            order_price: user.order_price,
            order_status: user.order_status,
            last_update: Some(MyDateTime(user.last_update.unwrap_or_default())),
        }
    }
}
