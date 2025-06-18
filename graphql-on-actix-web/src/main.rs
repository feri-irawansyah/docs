use actix_web::{http, web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;
use sqlx::PgPool;

use crate::{connection::db, schema::schema::{create_schema, AppSchema}};

mod connection {
    pub mod db;
}
mod schema {
    pub mod schema;
    pub mod datetime;
}
mod models {
    pub mod user_model;
}

mod services {
    pub mod user_service;
}

async fn graphql_handler(
    schema: web::Data<AppSchema>, 
    pool: web::Data<PgPool>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(pool.get_ref().clone())) // Inject disini bro!
        .await
        .into()
}

async fn graphiql() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/query").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::get_pg_pool().await;
    let schema = create_schema();

    HttpServer::new(move || {
    let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![http::header::CONTENT_TYPE])
        .max_age(3600)
        .supports_credentials();

    App::new()
        .app_data(web::Data::new(schema.clone()))
        .app_data(web::Data::new(pool.clone()))
        .route("/query", web::post().to(graphql_handler))
        .route("/graphql", web::get().to(graphiql))
        .wrap(cors) // cors udah didefinisikan di sini langsung
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

