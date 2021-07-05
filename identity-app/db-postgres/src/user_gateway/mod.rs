use async_trait::async_trait;
use tokio_postgres::Client;
use domain::entity::user::User;
use std::borrow::Borrow;

mod mutation;
mod query;

pub struct UserRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::UserDbGateway for UserRepository {
    async fn exists_by_username(&self, username: String) -> bool {
        let result = query::find_one_by_username(&(*self).client, username.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return false;
        }
        let row = result.unwrap();
        let name_found: String = row.get("username");
        println!("ROW IS {}", username);
        name_found == username
    }

    async fn insert(&self, user: &User) -> bool {
        let save_identity__user = mutation::save_identity__user(
            &(*self).client, user).await;

        let save_identity__user_username = mutation::save_identity__user_username(
            &(*self).client, user).await;

        let save_identity__user_email = mutation::save_identity__user_email(
            &(*self).client, user).await;

        let save_identity__user_phone = mutation::save_identity__user_phone(
            &(*self).client, user).await;

        let save_identity__user_enabled = mutation::save_identity__user_enabled(
            &(*self).client, user).await;

        return save_identity__user.is_ok() &&
            save_identity__user_username.is_ok() &&
            save_identity__user_email.is_ok() &&
            save_identity__user_phone.is_ok() &&
            save_identity__user_enabled.is_ok();
    }
}
