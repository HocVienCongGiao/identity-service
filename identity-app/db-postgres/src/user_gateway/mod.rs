use async_trait::async_trait;
use domain::boundaries::{DbError, UserDbResponse};
use domain::entity::user::User;
use tokio_postgres::{Client, Row};
use uuid::Uuid;

use crate::user_gateway::query::get_user_by_id;

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

    async fn insert(&self, user: &User) -> Result<(), DbError> {
        println!("Start insert user to db");

        let save_identity_user = mutation::save_identity_user(&(*self).client, user).await;
        if save_identity_user.is_err() {
            return Err(DbError::UnknownError);
        }

        let save_identity_user_username =
            mutation::save_identity_user_username(&(*self).client, user).await;

        if save_identity_user_username.is_err() {
            return Err(DbError::UniqueConstraintViolationError(
                "user_name".to_string(),
            ));
        }

        let save_identity_user_email =
            mutation::save_identity_user_email(&(*self).client, user).await;
        if save_identity_user_email.is_err() {
            return Err(DbError::UniqueConstraintViolationError("email".to_string()));
        }

        let save_identity_user_phone =
            mutation::save_identity_user_phone(&(*self).client, user).await;
        if save_identity_user_phone.is_err() {
            return Err(DbError::UniqueConstraintViolationError("phone".to_string()));
        }

        let save_identity_user_enabled =
            mutation::save_identity_user_enabled(&(*self).client, user).await;
        if save_identity_user_enabled.is_err() {
            return Err(DbError::UnknownError);
        }

        Ok(())
    }

    async fn deactivate_user(&self, id: Uuid) -> Result<User, DbError> {
        let deactivate_user = mutation::deactivate_identity_user(&(*self).client, id).await;
        println!("deactivate_user_result: {}", deactivate_user.is_ok());

        return if deactivate_user.is_err() {
            Err(DbError::UnknownError)
        } else {
            let user = get_user_by_id(&(*self).client, id).await.unwrap();
            Ok(User {
                id: user.get("id"),
                username: user.get("username"),
                email: user.get("email"),
                phone: user.get("phone"),
                enabled: user.get("enabled"),
            })
        };
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DbError> {
        let result = query::get_user_by_id(&(*self).client, id).await;

        println!("get_user_by_id: {}", result.is_ok());
        if result.is_err() {
            return Err(DbError::UnknownError)
        }

        let user = result.unwrap();
        return Ok(User {
            id: user.get("id"),
            username: user.get("username"),
            email: user.get("email"),
            phone: user.get("phone"),
            enabled: user.get("enabled"),
        })
    }
}
