use async_graphql::{dataloader::DataLoader, http::GraphiQLSource, *};
use actix_web::*;
use async_graphql_actix_web::*;
use sqlx::*;

use crate::{handlers::{order_handler::{ OrderMutation, OrderQuery }, user_handler::{UserMutation, UserQuery}}, services::order_service::OrderLoader};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    OrderQuery,
    UserQuery
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation, // mutation object,
    OrderMutation
);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
}

pub async fn graphiql() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/query").finish())
}

pub async fn graphql_handler(
    schema: web::Data<AppSchema>, 
    pool: web::Data<PgPool>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    let order_loader: DataLoader<OrderLoader> = DataLoader::new(OrderLoader { pool: pool.get_ref().clone() }, tokio::spawn);
    schema.execute(req.into_inner().data(pool.get_ref().clone()).data(order_loader)) // Inject disini bro!
        .await
        .into()
}
