use async_trait::async_trait;
use domain::boundaries::{DbError, UserCollectionDbResponse, UserDbResponse, UserQueryResponse};
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
    async fn activate_user(&self, id: Uuid) -> Result<User, DbError> {
        let activate_user = mutation::activate_identity_user(&(*self).client, id).await;
        println!("activate_user_result: {}", activate_user.is_ok());

        if activate_user.is_err() {
            return Err(DbError::UnknownError);
        }

        let user = get_user_by_id(&(*self).client, id).await.unwrap();
        Ok(User {
            id: user.get("id"),
            username: user.get("username"),
            email: user.get("email"),
            phone: user.get("phone"),
            enabled: user.get("enabled"),
        })
    }

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

        if deactivate_user.is_err() {
            return Err(DbError::UnknownError);
        }

        let user = get_user_by_id(&(*self).client, id).await.unwrap();
        Ok(User {
            id: user.get("id"),
            username: user.get("username"),
            email: user.get("email"),
            phone: user.get("phone"),
            enabled: user.get("enabled"),
        })
    }

    async fn get_user_by_id(&self, id: Uuid) -> Option<UserDbResponse> {
        let result = query::get_user_by_id(&(*self).client, id.clone()).await;

        println!("second block_on for row");
        if result.is_err() {
            return None;
        }
        println!("user query input boundary {}", id.to_hyphenated());

        let row = result.unwrap();
        println!("get_user_by_id : {:?}", row);

        Some(convert_to_user_db_response(row))
    }

    async fn get_users(
        &self,
        username: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        enabled: Option<bool>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> UserCollectionDbResponse {
        let filter = combine_into_filter_string(username, phone, email, enabled);
        let pagination = combine_into_pagination_string(offset, count);
        let result = query::get_users(&(*self).client, filter.clone(), pagination).await;

        let collection: Vec<UserDbResponse>;
        if result.is_err() {
            collection = vec![];
        } else {
            collection = result
                .unwrap()
                .into_iter()
                .map(|row| convert_to_user_db_response(row))
                .collect();
        }

        let has_more: Option<bool>;
        if let Some(count_param) = count {
            let pagination = combine_into_pagination_string(offset, None);
            let count_result = query::count_without_limit(&(*self).client, filter, pagination)
                .await
                .unwrap();
            if (count_result as u16) > count_param {
                has_more = Some(true);
            } else {
                has_more = Some(false);
            }
        } else {
            has_more = None
        };
        UserCollectionDbResponse {
            collection,
            has_more,
        }
    }
}

fn combine_into_filter_string(
    username: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    enabled: Option<bool>,
) -> String {
    let query_username = username
        .clone()
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());
    let query_phone = phone
        .clone()
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());
    let query_email = email
        .clone()
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());
    let query_enabled = enabled
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());

    if !username.clone().unwrap_or("".to_string()).is_empty()
        && !phone.clone().unwrap_or("".to_string()).is_empty()
        && !email.clone().unwrap_or("".to_string()).is_empty()
    {
        format!(
            "username LIKE '{}' AND \
            phone LIKE '{}' \
            AND email LIKE '{}'",
            query_username, query_phone, query_email
        )
    } else if !username.clone().unwrap_or("".to_string()).is_empty()
        && !phone.clone().unwrap_or("".to_string()).is_empty()
    {
        format!(
            "username LIKE '{}' AND \
            phone LIKE '{}'",
            query_username, query_phone
        )
    } else if !username.clone().unwrap_or("".to_string()).is_empty()
        && !email.clone().unwrap_or("".to_string()).is_empty()
    {
        format!(
            "username LIKE '{}' AND \
            email LIKE '{}'",
            query_username, query_email
        )
    } else if !phone.clone().unwrap_or("".to_string()).is_empty()
        && !email.clone().unwrap_or("".to_string()).is_empty()
    {
        format!(
            "phone LIKE '{}' AND \
            email LIKE '{}'",
            query_phone, query_email
        )
    } else if !phone.clone().unwrap_or("".to_string()).is_empty() {
        format!("phone LIKE '{}'", query_phone)
    } else if !email.clone().unwrap_or("".to_string()).is_empty() {
        format!("email LIKE '{}'", query_phone)
    } else {
        format!("username LIKE '{}'", query_username)
    }
}

fn combine_into_pagination_string(offset: Option<u16>, count: Option<u16>) -> String {
    let count = count
        .map(|value| value.to_string())
        .unwrap_or("ALL".to_string());
    let offset = offset.unwrap_or(0);

    format!("LIMIT {} OFFSET {}", count, offset)
}

fn convert_to_user_db_response(row: Row) -> UserDbResponse {
    let id: Uuid = row.get("id");
    let username: String = row.get("username");
    let phone: String = row.get("phone");
    let email: String = row.get("email");
    let enabled: bool = row.get("enabled");

    UserDbResponse {
        id,
        username,
        email,
        phone,
        enabled,
    }
}
