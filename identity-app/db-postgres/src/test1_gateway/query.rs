use tokio_postgres::{Client, Error, Row};
use tokio_postgres::types::ToSql;

pub async fn find_one_by_name(client: &Client, name: String) -> Result<Row, Error> {
    let stmt = 
         (*client)
        .prepare("SELECT * FROM author_initial WHERE name = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&name];
    client.query_one(&stmt, name_param).await
}