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

pub async fn find_one_by_group_name(client: &Client, group_name: String) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM identity__group WHERE group_name = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&group_name];
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

pub async fn get_group_by_group_name(client: &Client, group_name: &str) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM identity__group WHERE LOWER(group_name) = LOWER($1)")
        .await
        .unwrap();

    let name_param: &[&(dyn ToSql + Sync)] = &[&group_name];
    client.query_one(&stmt, name_param).await
}

pub async fn get_user_group_by_user_id(client: &Client, user_id: Uuid) -> Result<Vec<Row>, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM identity__user_group WHERE user_id = $1")
        .await
        .unwrap();

    let name_param: &[&(dyn ToSql + Sync)] = &[&user_id];
    client.query(&stmt, name_param).await
}

pub async fn get_groups_by_group_id(
    client: &Client,
    mut group_ids: Vec<Uuid>,
) -> Result<Vec<Row>, Error> {
    let mut filter_condition: String = "".to_string();
    let mut last_group_id = &group_ids.last().unwrap();
    for id in &group_ids {
        filter_condition.push_str(&*"'".to_string());
        filter_condition.push_str(&*id.to_simple().to_string());
        filter_condition.push_str(&*"'".to_string());
        if id.ne(last_group_id) {
            filter_condition.push_str(&*",".to_string());
        }
    }

    println!("final_group_ids_condition: {}", filter_condition);
    let filter = format!("id IN ({})", filter_condition,);

    let statement = format!(
        "SELECT * FROM identity__group \
        WHERE {} \
        ORDER BY id",
        filter
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[];
    client.query(&stmt, name_param).await
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
