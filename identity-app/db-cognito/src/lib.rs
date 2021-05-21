use chrono::prelude::*;
use rusoto_cognito_idp::{
    AdminCreateUserRequest, AdminDeleteUserRequest, CognitoIdentityProvider,
    CognitoIdentityProviderClient, ListUsersRequest,
};
use rusoto_core::{Client, Region};
use std::time::SystemTime;

async fn cognito() {
    // let dispatcher = HttpClient::new().expect("failed to create request dispatcher");
    // let default_provider_result = ProfileProvider::new();
    // let mut default_provider = default_provider_result.unwrap();
    // default_provider.set_profile("hvcg");
    // let aws_client = Client::new_with(default_provider, dispatcher);
    let aws_client = Client::shared();
    let user_pool_id = "ap-southeast-1_vmFHg7JIC".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    // Create a normal DateTime from the NaiveDateTime
    let now_datetime: DateTime<Utc> = SystemTime::now().into();

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

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn crud_users() {
        crate::cognito().await;
        assert_eq!(2 + 2, 4);
    }
}
