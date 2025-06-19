use actix_web::*;

use crate::{connection::db, graphql::schema::{create_schema, graphiql, graphql_handler}};

mod connection {
    pub mod db;
}

mod graphql {
    pub mod schema;
    pub mod datetime;
}

mod handlers {
    pub mod order_handler;
    pub mod user_handler;
}
mod models {
    pub mod user_model;
    pub mod order_model;
}
mod services {
    pub mod order_service;
    pub mod user_service;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;
    let schema = create_schema();

    HttpServer::new(move || {

    let cors = actix_cors::Cors::default()
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
    .allowed_headers(vec![http::header::CONTENT_TYPE])
    .max_age(3600)
    .supports_credentials();

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .app_data(web::Data::new(schema.clone()))
        .route("/query", web::post().to(graphql_handler))
        .route("/console/graphql", web::get().to(graphiql))
        .wrap(cors) // cors udah didefinisikan di sini langsung
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}