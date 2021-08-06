use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

pub async fn find_one_by_username(client: &Client, username: String) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM identity__user_username WHERE username = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&username];
    client.query_one(&stmt, name_param).await
}

pub async fn get_user_by_id(client: &Client, id: Uuid) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM identity__user_view WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&id];
    client.query_one(&stmt, name_param).await
}
