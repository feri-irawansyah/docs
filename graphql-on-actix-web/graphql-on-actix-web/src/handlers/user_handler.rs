use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::models::user_model::{NewUser, UpdateUser, User, UserWithOrders};
use crate::services::user_service::UserService;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = UserService::get_users(pool).await?;
        Ok(users)
    }
    // async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserWithOrders>> {
    //     let pool = ctx.data::<PgPool>()?;
    //     let users = UserService::get_users_with_orders(pool).await?;
    //     Ok(users)
    // }

    async fn user(&self, ctx: &Context<'_>, user_id: i32) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = UserService::get_user(pool, user_id).await?;
        Ok(user)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, request: NewUser) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = UserService::create_user(pool, request).await?;
        Ok(Some(user))
    }

    async fn update_user(&self, ctx: &Context<'_>, request: UpdateUser) -> Result<Option<User>> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let order = UserService::update_user(pool, request).await?;
        Ok(order)
    }
}