mod connection;
mod query;

pub use connection::*;
pub use query::*;

#[tokio::test]
async fn create_table_in_sql_server() {
    let result = create_table().await;
    assert_eq!(result.is_ok(), true);
}