use crate::user_gateway::query::{
    get_groups_by_group_id, get_user_by_id, get_user_group_by_user_id,
};
use async_trait::async_trait;
use domain::boundaries::{
    DbError, GroupDbResponse, UserCollectionDbResponse, UserDbResponse, UserQueryResponse,
};
use domain::entity::user::User;
use tokio_postgres::{Client, Row};
use uuid::Uuid;

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

        let user = query::get_user_by_id(&(*self).client, id).await;
        println!("activate_user_result_db: {:?}", user);
        let row = user.unwrap();

        if row.is_empty() {
            return Err(DbError::UnknownError);
        }

        let user = get_user_by_id(&(*self).client, id).await.unwrap();
        // get user group by user id
        let mut user_groups = get_user_group_by_user_id(&(*self).client, id)
            .await
            .unwrap();
        if user_groups.is_empty() {
            println!("User group is empty");
            return Err(DbError::UnknownError);
        }
        let mut group_ids = vec![];
        for row in user_groups {
            let group_id = row.get("group_id");
            group_ids.push(group_id);
        }
        let mut groups = get_groups_by_group_id(&(*self).client, group_ids)
            .await
            .unwrap();
        if groups.is_empty() {
            println!("Groups is empty");
            return Err(DbError::UnknownError);
        }
        let mut group_names: Vec<String> = vec![];
        for group in groups {
            let group_name = group.get("group_name");
            group_names.push(group_name)
        }

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            phone: row.get("phone"),
            enabled: row.get("enabled"),
            group: Option::from(group_names),
        })
    }

    async fn deactivate_user(&self, id: Uuid) -> Result<User, DbError> {
        let deactivate_user = mutation::deactivate_identity_user(&(*self).client, id).await;
        println!("deactivate_user_result: {}", deactivate_user.is_ok());

        if deactivate_user.is_err() {
            return Err(DbError::UnknownError);
        }

        let user = get_user_by_id(&(*self).client, id).await.unwrap();
        // get user group by user id
        let mut user_groups = get_user_group_by_user_id(&(*self).client, id)
            .await
            .unwrap();
        if user_groups.is_empty() {
            println!("User group is empty");
            return Err(DbError::UnknownError);
        }
        let mut group_ids = vec![];
        for row in user_groups {
            let group_id = row.get("group_id");
            group_ids.push(group_id);
        }
        println!("group_ids: {:?}", group_ids);
        let mut groups = get_groups_by_group_id(&(*self).client, group_ids)
            .await
            .unwrap();
        println!("mut_groups_result: {:?}", groups);
        if groups.is_empty() {
            println!("Groups is empty");
            return Err(DbError::UnknownError);
        }
        let mut group_names: Vec<String> = vec![];
        for group in groups {
            let group_name: String = group.get("group_name");
            group_names.push(group_name)
        }

        Ok(User {
            id: user.get("id"),
            username: user.get("username"),
            email: user.get("email"),
            phone: user.get("phone"),
            enabled: user.get("enabled"),
            group: Option::from(group_names),
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

    async fn get_group_by_group_name(&self, group_name: &str) -> Option<GroupDbResponse> {
        let result = query::get_group_by_group_name(&(*self).client, group_name).await;

        println!("second block_on for row");
        if result.is_err() {
            return None;
        }
        println!("user query input boundary {}", group_name);

        let row = result.unwrap();
        println!("get_group_by_group_name : {:?}", row);

        Some(convert_to_group_db_response(row))
    }

    async fn insert(&self, user: &User, group_ids: Vec<Uuid>) -> Result<(), DbError> {
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

        let save_identity_user_group =
            mutation::save_identity_user_group(&(*self).client, user, group_ids).await;
        if !save_identity_user_group {
            return Err(DbError::UnknownError);
        }

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Option<UserDbResponse> {
        let result = query::get_user_by_id(&(*self).client, id).await;

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
        collection = result
            .unwrap()
            .into_iter()
            .map(convert_to_user_db_response)
            .collect();

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

    async fn update(&self, user: &User, group_ids: Vec<Uuid>) -> Result<(), DbError> {
        println!("Start update user to db");
        if !user.username.is_empty() {
            let update_identity_username =
                mutation::update_identity_user_username(&(*self).client, user).await;
            println!(
                "update_identity_user_username result: {:?}",
                update_identity_username
            );
            if update_identity_username.is_err() {
                return Err(DbError::UnknownError);
            }
        }

        if user.phone.is_some() {
            let update_identity_phone =
                mutation::update_identity_user_phone(&(*self).client, user).await;
            println!(
                "update_identity_user_phone result: {:?}",
                update_identity_phone
            );
            if update_identity_phone.is_err() {
                return Err(DbError::UnknownError);
            }
        }

        if user.email.is_some() {
            let update_identity_email =
                mutation::update_identity_user_email(&(*self).client, user).await;
            println!(
                "update_identity_user_email result: {:?}",
                update_identity_email
            );
            if update_identity_email.is_err() {
                return Err(DbError::UnknownError);
            }
        }

        if user.group.is_some() {
            let update_identity_group =
                mutation::update_identity_user_group(&(*self).client, user, group_ids).await;
            if !update_identity_group {
                return Err(DbError::UnknownError);
            }
        }

        Ok(())
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
        .unwrap_or_else(|| "%".to_string());
    let query_phone = phone
        .clone()
        .map(|value| format!("%{}%", value))
        .unwrap_or_else(|| "%".to_string());
    let query_email = email
        .clone()
        .map(|value| format!("%{}%", value))
        .unwrap_or_else(|| "%".to_string());
    let query_enabled = enabled
        .map(|value| format!("%{}%", value))
        .unwrap_or_else(|| "%".to_string());

    if is_search_username_phone_email(username.clone(), phone.clone(), email.clone()) {
        format!(
            "username LIKE '{}' AND \
            phone LIKE '{}' \
            AND email LIKE '{}'",
            query_username, query_phone, query_email
        )
    } else if is_search_username_phone(username.clone(), phone.clone()) {
        format!(
            "username LIKE '{}' AND \
            phone LIKE '{}'",
            query_username, query_phone
        )
    } else if is_search_username_email(username, email.clone()) {
        format!(
            "username LIKE '{}' AND \
            email LIKE '{}'",
            query_username, query_email
        )
    } else if is_search_phone_email(phone.clone(), email.clone()) {
        format!(
            "phone LIKE '{}' AND \
            email LIKE '{}'",
            query_phone, query_email
        )
    } else if !phone.unwrap_or_else(|| "".to_string()).is_empty() {
        format!("phone LIKE '{}'", query_phone)
    } else if !email.unwrap_or_else(|| "".to_string()).is_empty() {
        format!("email LIKE '{}'", query_phone)
    } else {
        format!("username LIKE '{}'", query_username)
    }
}

fn combine_into_pagination_string(offset: Option<u16>, count: Option<u16>) -> String {
    let count = count
        .map(|value| value.to_string())
        .unwrap_or_else(|| "ALL".to_string());
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
        group: vec![],
    }
}

fn convert_to_group_db_response(row: Row) -> GroupDbResponse {
    let id: Uuid = row.get("id");
    let group_name: String = row.get("group_name");

    GroupDbResponse { id, group_name }
}

fn is_search_username_phone_email(
    username: Option<String>,
    phone: Option<String>,
    email: Option<String>,
) -> bool {
    !username.unwrap_or_else(|| "".to_string()).is_empty()
        && !phone.unwrap_or_else(|| "".to_string()).is_empty()
        && !email.unwrap_or_else(|| "".to_string()).is_empty()
}

fn is_search_username_phone(username: Option<String>, phone: Option<String>) -> bool {
    !username.unwrap_or_else(|| "".to_string()).is_empty()
        && !phone.unwrap_or_else(|| "".to_string()).is_empty()
}

fn is_search_username_email(username: Option<String>, email: Option<String>) -> bool {
    !username.unwrap_or_else(|| "".to_string()).is_empty()
        && !email.unwrap_or_else(|| "".to_string()).is_empty()
}

fn is_search_phone_email(phone: Option<String>, email: Option<String>) -> bool {
    !phone.unwrap_or_else(|| "".to_string()).is_empty()
        && !email.unwrap_or_else(|| "".to_string()).is_empty()
}
