use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::models::user_model::User;
use crate::services::user_service::UserService;

#[derive(Default)]
pub struct UserHandler;

#[Object]
impl UserHandler {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = UserService::get_users(pool).await?;
        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, user_id: i32) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = UserService::get_user(pool, user_id).await?;
        Ok(user)
    }
}
