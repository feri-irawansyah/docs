use async_graphql::{http::GraphiQLSource, *};
use actix_web::*;
use async_graphql_actix_web::*;
use sqlx::*;

use crate::handlers::order_handler::OrderHandler;

#[derive(MergedObject, Default)]
pub struct ApplicationRoot(
    OrderHandler,
);

pub type AppSchema = Schema<ApplicationRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(ApplicationRoot::default(), EmptyMutation, EmptySubscription)
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
    schema
        .execute(req.into_inner().data(pool.get_ref().clone())) // Inject disini bro!
        .await
        .into()
}
