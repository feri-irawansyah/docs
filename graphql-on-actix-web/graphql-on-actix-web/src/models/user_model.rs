use async_graphql::{dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{models::order_model::Order, services::order_service::OrderLoader};

#[derive(Debug, Deserialize, InputObject)]
pub struct NewUser {
    pub email: String,
    pub full_name: String
}

#[derive(Debug, Deserialize, Serialize, InputObject)]
pub struct UpdateUser {
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct UserWithOrderRow {
    pub user_id: i32,
    pub user_email: String,
    pub user_full_name: String,
    pub order_id: Option<i32>,
    pub order_name: Option<String>,
    pub order_date: Option<chrono::DateTime<chrono::Utc>>,
    pub order_price: Option<f64>,
    pub order_status: Option<String>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(SimpleObject)]
pub struct UserWithOrders {
    pub user_id: i32,
    pub email: String,
    pub full_name: String,
    pub orders: Vec<Order>,
}

#[derive(FromRow, Deserialize, Debug, Serialize)]
pub struct UserDB {
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}

impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        User {
            user_id: user.user_id,
            email: user.email,
            full_name: user.full_name
        }
    }
}

#[ComplexObject]
impl User {
    async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<Order>, async_graphql::Error> {
        let loader = ctx.data::<DataLoader<OrderLoader>>()?;
        let orders = loader.load_one(self.user_id).await?;
        Ok(orders.unwrap_or_default())
    }
}