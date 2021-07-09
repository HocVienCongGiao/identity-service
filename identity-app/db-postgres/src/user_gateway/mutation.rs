use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};

use domain::entity::user::User;

pub async fn save_identity_user(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into identity__user(id) VALUES ($1)")
        .await
        .unwrap();
    println!("id: {}", user.id);

    let params: &[&(dyn ToSql + Sync)] = &[&user.id];
    client.execute(&stmt, params).await
}

pub async fn save_identity_user_username(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into identity__user_username(id, username) VALUES ($1, $2)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&user.id, &user.username];
    client.execute(&stmt, params).await
}

pub async fn save_identity_user_email(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into identity__user_email(id, email) VALUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&user.id, &user.email];

    client.execute(&stmt, params).await
}

pub async fn save_identity_user_phone(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into identity__user_phone(id, phone) VALUES ($1, $2)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&user.id, &user.phone];
    client.execute(&stmt, params).await
}

pub async fn save_identity_user_enabled(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into identity__user_enabled(id, enabled) VALUES ($1, true)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&user.id];
    client.execute(&stmt, params).await
}
