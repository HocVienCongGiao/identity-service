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

    let name_param: &[&(dyn ToSql + Sync)] = &[&id];
    client.query_one(&stmt, name_param).await
}

pub async fn get_users(
    client: &Client,
    filter: String,
    pagination: String,
) -> Result<Vec<Row>, Error> {
    let statement = format!(
        "SELECT * FROM identity__user_view \
        WHERE {} \
        ORDER BY id \
        {}",
        filter, pagination
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[];
    client.query(&stmt, name_param).await
}

pub async fn count_without_limit(
    client: &Client,
    filter: String,
    pagination: String,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM
        (SELECT * FROM identity__user_view \
        WHERE {} \
        ORDER BY id \
        {}) AS users",
        filter, pagination
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}
