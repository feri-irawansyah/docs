use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct OrderHandler;

#[Object]
impl OrderHandler {
    async fn orders(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"orders": "orders"}))
    }
}
