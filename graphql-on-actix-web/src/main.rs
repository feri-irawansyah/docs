use actix_web::*;
use async_graphql::http::GraphiQLSource;
use sqlx::*;
use async_graphql::*;
use async_graphql_actix_web::*;

use crate::connection::db;

mod connection {
    pub mod db;
}

#[derive(MergedObject, Default)]
pub struct ApplicationRoot(
    
   
);

async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let value: i32 = row.get(0);
            HttpResponse::Ok().body(format!("Query returned: {}", value))
        },
        Err(err) => {
            eprintln!("DB error: {}", err);
            HttpResponse::InternalServerError().body("DB error")
        }
    }
}

async fn graphiql() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/query").finish())
}

pub type AppSchema = Schema<ApplicationRoot, EmptyMutation, EmptySubscription>;

fn create_schema() -> AppSchema {
    Schema::build(ApplicationRoot::default(), EmptyMutation, EmptySubscription)
        .finish()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;

    HttpServer::new(move || {

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .route("/", web::get().to(hello))
        .route("/console/graphql", web::get().to(graphiql))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}