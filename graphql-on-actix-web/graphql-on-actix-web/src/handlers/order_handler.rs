use async_graphql::{Context, Object, Result};

use crate::{models::order_model::{NewOrder, Order, UpdateOrder}, services::order_service::OrderService};

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

    async fn update_order(&self, ctx: &Context<'_>, request: UpdateOrder) -> Result<Option<Order>> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let order = OrderService::update_order(pool, request).await?;
        Ok(order)
    }

    async fn delete_order(&self, ctx: &Context<'_>, order_id: i32) -> Result<bool> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let result = OrderService::delete_order(pool, order_id).await?;
        Ok(result)
    }
}
