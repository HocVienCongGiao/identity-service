use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error};

use domain::entity::user::User;
use uuid::Uuid;

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

pub async fn deactivate_identity_user(client: &Client, id: Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE identity__user_enabled SET enabled = false where id = $1")
        .await
        .unwrap();
    println!("id: {}", id);

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub async fn activate_identity_user(client: &Client, id: Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE identity__user_enabled SET enabled = true where id = $1")
        .await
        .unwrap();
    println!("id: {}", id);

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub async fn update_identity_user_username(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE identity__user_username SET username = $1 where id = $2")
        .await
        .unwrap();
    println!("update_identity_user_username id: {}", user.id);

    let params: &[&(dyn ToSql + Sync)] = &[&user.username, &user.id];
    client.execute(&stmt, params).await
}

pub async fn update_identity_user_phone(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE identity__user_phone SET phone = $1 where id = $2")
        .await
        .unwrap();
    println!("update_identity_user_username id: {}", user.id);

    let params: &[&(dyn ToSql + Sync)] = &[&user.phone, &user.id];
    client.execute(&stmt, params).await
}

pub async fn update_identity_user_email(client: &Client, user: &User) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE identity__user_email SET email = $1 where id = $2")
        .await
        .unwrap();
    println!("update_identity_user_username id: {}", user.id);

    let params: &[&(dyn ToSql + Sync)] = &[&user.email, &user.id];
    client.execute(&stmt, params).await
}

pub async fn save_identity_user_group(client: &Client, user: &User, group_ids: Vec<Uuid>) -> bool {
    for group_id in group_ids {
        let user_group_id = Uuid::new_v4();
        println!("user_group_id: {}", user_group_id);
        let stmt = (*client)
            .prepare("INSERT into identity__user_group(id, user_id, group_id) VALUES ($1, $2, $3)")
            .await
            .unwrap();
        let params: &[&(dyn ToSql + Sync)] = &[&user_group_id, &user.id, &group_id];
        let result = client.execute(&stmt, params).await;
        if result.is_err() {
            return false;
        }
    }
    true
}

pub async fn update_identity_user_group(
    client: &Client,
    user: &User,
    group_ids: Vec<Uuid>,
) -> bool {
    // Remove existing group with this customer id
    let delete_user_group_result = delete_user_group_by_user_id(client, &user.id).await;
    if delete_user_group_result.is_err() {
        print!(
            "Error when delete user group: {:?}",
            delete_user_group_result
        );
        return false;
    }
    for group_id in group_ids {
        let user_group_id = Uuid::new_v4();
        println!("user_group_id: {}", user_group_id);
        let stmt = (*client)
            .prepare("INSERT into identity__user_group(id, user_id, group_id) VALUES ($1, $2, $3)")
            .await
            .unwrap();
        let params: &[&(dyn ToSql + Sync)] = &[&user_group_id, &user.id, &group_id];
        let result = client.execute(&stmt, params).await;
        if result.is_err() {
            return false;
        }
    }
    true
}

pub async fn delete_user_group_by_user_id(client: &Client, user_id: &Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("DELETE FROM identity__user_group WHERE user_id = $1")
        .await
        .unwrap();
    println!("delete_user_group_by_user_id id: {}", &user_id);

    let params: &[&(dyn ToSql + Sync)] = &[&user_id];
    client.execute(&stmt, params).await
}
