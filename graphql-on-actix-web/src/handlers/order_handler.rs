use async_graphql::{Context, Object, Result};

use crate::{models::order_model::{NewOrder, Order}, services::order_service::OrderService};

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn orders(&self, ctx: &Context<'_>, user_id: i32) -> Result<Vec<Order>> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let orders = OrderService::get_orders(pool, user_id).await?;
        Ok(orders)
    }
}

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
     async fn create_order(&self, ctx: &Context<'_>, request: NewOrder) -> Result<Order> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let order = OrderService::create_order(pool, request).await?;
        Ok(order)
    }
}
