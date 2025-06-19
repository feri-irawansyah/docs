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
}
mod models {
    pub mod user_model;
}
mod services {
    pub mod order_service;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;
    let schema = create_schema();

    HttpServer::new(move || {

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .app_data(web::Data::new(schema.clone()))
        .route("/query", web::post().to(graphql_handler))
        .route("/console/graphql", web::get().to(graphiql))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}