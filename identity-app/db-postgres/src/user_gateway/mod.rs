use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entity::user::User;

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
        println!("Start insert user to db");

        let save_identity_user = mutation::save_identity_user(&(*self).client, user).await;
        if save_identity_user.is_err() {
            panic!(
                "Problem insert identity user: {:?}",
                save_identity_user.err()
            )
        }

        let save_identity_user_username =
            mutation::save_identity_user_username(&(*self).client, user).await;
        if save_identity_user_username.is_err() {
            panic!(
                "Problem insert identity user name : {:?}",
                save_identity_user_username.err()
            )
        }

        let save_identity_user_email =
            mutation::save_identity_user_email(&(*self).client, user).await;
        if save_identity_user_email.is_err() {
            panic!(
                "Problem insert identity email : {:?}",
                save_identity_user_email.err()
            )
        }

        let save_identity_user_phone =
            mutation::save_identity_user_phone(&(*self).client, user).await;
        if save_identity_user_phone.is_err() {
            panic!(
                "Problem insert identity phone : {:?}",
                save_identity_user_phone.err()
            )
        }

        let save_identity_user_enabled =
            mutation::save_identity_user_enabled(&(*self).client, user).await;
        if save_identity_user_enabled.is_err() {
            panic!(
                "Problem insert identity enabled : {:?}",
                save_identity_user_enabled.err()
            )
        }

        return save_identity_user.is_ok()
            && save_identity_user_username.is_ok()
            && save_identity_user_email.is_ok()
            && save_identity_user_phone.is_ok()
            && save_identity_user_enabled.is_ok();
    }
}
