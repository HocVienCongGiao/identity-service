use std::time::SystemTime;

use chrono::prelude::*;
use rusoto_cognito_idp::{AdminCreateUserRequest, AdminDeleteUserRequest, CognitoIdentityProvider, CognitoIdentityProviderClient, ListUsersRequest, AttributeType};
use rusoto_core::{Client, Region};
use hvcg_iam_openapi_identity::models::User;
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};

pub async fn insert_cognito_user() {
    // let dispatcher = HttpClient::new().expect("failed to create request dispatcher");
    // let default_provider_result = ProfileProvider::new();
    // let mut default_provider = default_provider_result.unwrap();
    // default_provider.set_profile("hvcg");
    // let aws_client = Client::new_with(default_provider, dispatcher);
    let aws_client = Client::shared();
    let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    // Create a normal DateTime from the NaiveDateTime
    let now_datetime: DateTime<Utc> = SystemTime::now().into();

    // TODO Can remove this code if not use
    // let deserialized_user: User = serde_json::from_slice(&*response_body).unwrap();
    // let user_attributes = set_user_attributes(deserialized_user);

    let test_username =
        "dev-test-user".to_string() + now_datetime.format("%H%M%S%f").to_string().as_str();

    let admin_create_user_request = AdminCreateUserRequest {
        desired_delivery_mediums: None,
        force_alias_creation: None,
        message_action: None,
        temporary_password: None,
        user_attributes: None,
        user_pool_id: user_pool_id.clone(),
        username: test_username.clone(),
        validation_data: None,
    };
    let _ = rusoto_cognito_idp_client
        .admin_create_user(admin_create_user_request)
        .sync();

    let list_user_request = ListUsersRequest {
        attributes_to_get: None,
        filter: None,
        limit: None,
        pagination_token: None,
        user_pool_id: user_pool_id.clone(),
    };
    match rusoto_cognito_idp_client
        .list_users(list_user_request)
        .sync()
    {
        Ok(response) => match response.users {
            Some(user_types) => {
                println!("User Type here");
                for user_type in user_types {
                    let naive = NaiveDateTime::from_timestamp(
                        user_type.user_create_date.unwrap() as i64,
                        0,
                    );

                    // Create a normal DateTime from the NaiveDateTime
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

                    // Format the datetime how you want
                    let user_created_date = datetime.format("%Y-%m-%d %H:%M:%S");

                    println!(
                        "Username: {} - Created at: {:?}",
                        user_type.username.unwrap(),
                        user_created_date
                    )
                }
            }
            None => println!("No buckets in region!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let admin_delete_user_request = AdminDeleteUserRequest {
        user_pool_id,
        username: test_username,
    };
    let _ = rusoto_cognito_idp_client
        .admin_delete_user(admin_delete_user_request)
        .sync();
}

fn set_user_attributes(deserialized_user: User) -> (Vec<AttributeType>) {
    let mut user_attributes = Vec::new();
    let email = AttributeType {
        name: "email".to_string(),
        value: Option::from(deserialized_user.email)
    };
    let email_verified = AttributeType {
        name: "email_verified".to_string(),
        value: Option::from("true".to_string())
    };

    let phone = AttributeType {
        name: "phone".to_string(),
        value: Option::from(deserialized_user.phone)
    };
    let phone_verified = AttributeType {
        name: "phone_verified".to_string(),
        value: Option::from("true".to_string())
    };

    user_attributes.push(email);
    user_attributes.push(email_verified);
    user_attributes.push(phone);
    user_attributes.push(phone_verified);
    user_attributes
}

#[cfg(test)]
mod tests {
    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::{Response, Body};

    #[tokio::test]
    async fn crud_users() {
        crate::insert_cognito_user().await;
        let result = 4;
        assert_eq!(result, 4);
    }
}
