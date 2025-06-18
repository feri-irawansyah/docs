use async_graphql::{Context, Object, Schema, Result, EmptyMutation, EmptySubscription};
use sqlx::PgPool;

use crate::{models::user_model::User, services::user_service::UserService};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello from Actix + GraphQL!"
    }

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

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish()
}
