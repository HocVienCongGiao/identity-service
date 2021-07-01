use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};

pub async fn save(
    client: &Client,
    user: User
) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare(
            "INSERT into user(username, email, phone, enabled) VALUES \
            ($1, $2, $3, true)"
        )
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&user.username, &user.email, &user.phone];
    client.execute(&stmt, params).await
}
